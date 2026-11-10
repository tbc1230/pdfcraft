//! Protect Using Password in the real shell (egui_kittest).

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

const FIXTURE: &[u8] = b"%PDF-1.7
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 300 200] >> endobj
trailer << /Root 1 0 R >>
%%EOF";

fn harness() -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(|_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.open_bytes("doc.pdf", None, FIXTURE.to_vec()).unwrap();
        app
    });
    h.run_steps(4);
    h
}

fn type_into(h: &mut Harness<'static, PdfCraftApp>, label: &str, text: &str) {
    h.get_by_label(label).click();
    h.run_steps(2);
    h.get_by_label(label).type_text(text);
    h.run_steps(2);
}

#[test]
fn protecting_for_viewing_requires_matching_passwords_and_saves_encrypted() {
    let mut h = harness();
    assert!(h.state_mut().execute("protect.password"));
    h.run_steps(2);
    assert_eq!(h.state().dialog, Some(Dialog::Protect));
    h.get_by_label("Viewing");
    type_into(&mut h, "Type Password", "Open-Sesame-42");
    type_into(&mut h, "Re-type Password", "Open-Sesame-4");
    h.get_by_label_contains("don't match");
    assert!(h.query_by(|n| n.label().as_deref() == Some("Apply") && n.is_disabled()).is_some(), "Apply is disabled");
    type_into(&mut h, "Re-type Password", "2");
    h.get_by_label("Strength: Strong");
    h.get_by_label("Apply").click();
    h.run_steps(3);
    assert_eq!(h.state().dialog, None);
    let id = h.state().views[0].id;
    let doc = h.state().session.get(id).unwrap();
    assert!(doc.info.encrypted && doc.dirty);
    assert!(doc.security_summary().unwrap().pending);
    // What Save writes needs the password.
    let bytes = h.state().session.save_bytes(id).unwrap();
    let mut s = pdfcraft_engine::Session::new();
    assert!(s.open("p.pdf", None, bytes.clone(), None).is_err());
    assert!(s.open("p.pdf", None, bytes, Some("Open-Sesame-42")).is_ok());
    // Remove security is now offered, and undo also takes the protection back.
    assert!(h.state_mut().execute("protect.remove"));
    h.state_mut().undo();
    h.state_mut().undo();
    assert!(!h.state().session.get(id).unwrap().info.encrypted);
}

#[test]
fn editing_restrictions_via_advanced_options() {
    let mut h = harness();
    h.state_mut().execute("protect.password");
    h.run_steps(2);
    h.get_by_label("Editing").click();
    h.run_steps(2);
    h.get_by_label("Advanced Options ⌄").click();
    h.run_steps(2);
    h.get_by_label("Enable copying of text, images, and other content").click();
    h.run_steps(1);
    type_into(&mut h, "Type Password", "boss");
    type_into(&mut h, "Re-type Password", "boss");
    h.get_by_label("Apply").click();
    h.run_steps(3);
    let id = h.state().views[0].id;
    let sec = h.state().session.get(id).unwrap().security_summary().unwrap();
    assert!(sec.permissions.copy() && sec.permissions.print() && !sec.permissions.modify() && !sec.permissions.annotate());
    // Remove security is disabled for nobody here: this session set it.
    assert!(h.state().session.get(id).unwrap().allows_security_change());
}

// -- Honour the restrictions set inside a PDF ---------------------------------------------------

/// An `n`-page document with a proper xref table.
fn pdf(n: usize) -> Vec<u8> {
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")));
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into());
    for i in 0..n {
        objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i));
        let body = format!("BT /F1 24 Tf 20 150 Td (Page {}) Tj ET", i + 1);
        objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()));
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

/// A two-page document encrypted with AES-256: user password `user`, owner password "owner", `/P`
/// as given.
fn locked(user: &str, permissions: i32) -> Vec<u8> {
    let mut doc = pdfcraft_cos::Document::open(std::sync::Arc::new(pdf(2))).unwrap();
    doc.set_encryption(&pdfcraft_cos::NewEncryption {
        algorithm: pdfcraft_cos::Algorithm::Aes256,
        user_password: user,
        owner_password: "owner",
        permissions,
        encrypt_metadata: true,
        seed: [4; 32],
    })
    .unwrap();
    pdfcraft_cos::write_full(&doc, &Default::default()).unwrap()
}

/// `/P` with the given bits (1-based, ISO 32000-2 Table 22) cleared.
fn without(bits: &[u32]) -> i32 {
    bits.iter().fold(-1, |p, bit| p & !(1 << (bit - 1)))
}

/// One document per right the editor state reports, with the bits that withhold that right alone.
/// Form filling is granted by bit 9 or bit 6, so both go.
const RIGHTS: [(&str, &[u32]); 8] = [
    ("print", &[3]),
    ("modify", &[4]),
    ("copy", &[5]),
    ("annotate", &[6]),
    ("fill_forms", &[6, 9]),
    ("extract_for_accessibility", &[10]),
    ("assemble", &[11]),
    ("print_high_quality", &[12]),
];

const NOTICE: &str = "This document is secured";

/// `bytes` open in a tab; `password` answers the prompt of a document that has a user password.
fn open_locked(bytes: Vec<u8>, password: Option<&'static str>) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.open_bytes("locked.pdf", None, bytes).unwrap();
        if let Some(password) = password {
            app.submit_password(Some(password.into()));
        }
        app
    });
    h.run_steps(4);
    h
}

fn type_password(h: &mut Harness<'static, PdfCraftApp>, password: &str) {
    let field = h.get_by_role(egui::accesskit::Role::PasswordInput);
    field.focus();
    field.type_text(password);
    h.run_steps(1);
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
}

fn add_rect(app: &mut PdfCraftApp) -> bool {
    let shape = pdfcraft_engine::Shape::Rectangle { rect: [10.0, 10.0, 50.0, 50.0] };
    let style = pdfcraft_engine::Style::default_for(&shape);
    app.apply_edit(pdfcraft_engine::Edit::AddAnnotation(pdfcraft_engine::NewAnnotation {
        page: 0,
        shape,
        style,
        contents: String::new(),
        author: String::new(),
    }))
}

/// The notice is shown for every document opened without the owner password that withholds any of
/// the eight rights, and for no other.
#[test]
fn the_notice_shows_for_each_withheld_right_and_only_then() {
    for (name, bits) in RIGHTS {
        let h = open_locked(locked("", without(bits)), None);
        let id = h.state().views[0].id;
        let summary = h.state().session.get(id).unwrap().security_summary().unwrap();
        assert!(summary.restricts_any_right(), "{name}: the rule sees the restriction");
        assert!(h.query_by_label_contains(NOTICE).is_some(), "{name}: the notice is shown");
    }
    // Encrypted with nothing withheld, and not encrypted at all.
    for (what, bytes) in [("nothing withheld", locked("", -1)), ("not encrypted", pdf(2))] {
        let h = open_locked(bytes, None);
        assert!(h.query_by_label_contains(NOTICE).is_none(), "{what}");
    }
    // Opened with the owner password, whatever /P withholds.
    let h = open_locked(locked("reader", without(&[4, 5])), Some("owner"));
    let id = h.state().views[0].id;
    assert!(!h.state().session.get(id).unwrap().security_summary().unwrap().restricts_any_right());
    assert!(h.query_by_label_contains(NOTICE).is_none(), "after the owner password was entered");
}

/// No "Don't show again", no Preferences switch, and an old stored `show_secured_notice: false`
/// hides nothing: the notice shows each time a restricted document is opened. Dismiss hides it for
/// the one document.
#[test]
fn the_notice_cannot_be_turned_off_for_good() {
    let bytes = locked("", without(&[4]));
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.restore(r#"{"show_secured_notice": false}"#);
        app.set_option("language", "en").unwrap();
        app.open_bytes("one.pdf", None, bytes).unwrap();
        app
    });
    h.run_steps(4);
    h.get_by_label_contains(NOTICE);
    assert!(h.query_by_label("Don't show again").is_none());
    assert!(!h.state().persist().contains("show_secured_notice"), "nothing about it is stored any more");
    h.get_by_label("Dismiss").click();
    h.run_steps(2);
    assert!(h.query_by_label_contains(NOTICE).is_none(), "Dismiss hides it for this document");
    h.state_mut().open_bytes("two.pdf", None, locked("", without(&[5]))).unwrap();
    h.run_steps(4);
    h.get_by_label_contains(NOTICE);
    assert!(h.state_mut().set_option("secured-notice", "false").is_err(), "no option turns it off");
    h.state_mut().set_option("dialog", "preferences").unwrap();
    h.run_steps(3);
    assert!(
        h.query_by_role_and_label(egui::accesskit::Role::CheckBox, "Show a notice when a document is secured").is_none(),
        "Preferences has no switch for it"
    );
}

/// The Security tab shows the rights, offers Unlock and asks for the password through the same
/// prompt as the notice.
#[test]
fn the_security_tab_unlock_asks_for_the_password() {
    let mut h = open_locked(locked("", without(&[5])), None);
    // The notice has an Unlock of its own: dismiss it, so the one left is the Security tab's.
    h.get_by_label("Dismiss").click();
    h.run_steps(2);
    assert!(h.query_by_label_contains(NOTICE).is_none());
    h.state_mut().set_option("dialog", "properties").unwrap();
    h.run_steps(2);
    h.get_by_label("Security").click();
    h.run_steps(3);
    h.get_by_label("User password");
    h.get_by_label("Unlock\u{2026}").click();
    h.run_steps(3);
    assert!(h.state().unlock_prompt.is_some());
    h.get_by_label_contains("Enter its permissions password");
}

/// One sweep: with each right withheld in turn, every registered command that needs that right is
/// refused with its reason and changes nothing. (The web build offers a subset of these commands;
/// the refusal is the same.)
#[test]
fn a_command_that_needs_a_withheld_right_is_refused_and_changes_nothing() {
    use pdfcraft_engine::commands::{COMMANDS, Needs};
    let needs_right = |right: &str, needs: Needs| match right {
        "print" => needs == Needs::Printing,
        "modify" => matches!(needs, Needs::Modification | Needs::Marks(_) | Needs::HasComments | Needs::HasFields | Needs::HasRedactions),
        "copy" => needs == Needs::Copying,
        "annotate" => needs == Needs::Annotate,
        "fill_forms" => needs == Needs::FillForms,
        "assemble" => needs == Needs::Assembly,
        // No registered command needs these two on their own.
        _ => false,
    };
    for (name, bits) in RIGHTS {
        let mut h = open_locked(locked("", without(bits)), None);
        let id = h.state().views[0].id;
        let state = |h: &Harness<'static, PdfCraftApp>| {
            let d = h.state().session.get(id).unwrap();
            (d.edit_generation(), d.dirty, d.info.pages.len(), d.info.annotations.len())
        };
        let before = state(&h);
        let mut swept = 0;
        for spec in COMMANDS.iter().filter(|c| needs_right(name, c.needs)) {
            h.state_mut().toast = None;
            assert!(!h.state_mut().execute(spec.id), "{name}: {} is refused", spec.id);
            assert!(h.state().toast.is_some(), "{name}: {} says why", spec.id);
            assert_eq!(state(&h), before, "{name}: {} changed nothing", spec.id);
            swept += 1;
        }
        let expected_some = matches!(name, "print" | "modify" | "copy" | "annotate" | "fill_forms" | "assemble");
        assert_eq!(swept > 0, expected_some, "{name}: {swept} commands");
    }
}

/// After Unlock with the owner password the rights are lifted, the notice is gone and the edits made
/// before are kept; a wrong password or the user password changes nothing and the prompt says so.
#[test]
fn unlocking_lifts_the_rights_and_keeps_the_edits() {
    // Printing and commenting stay allowed, so there is an edit to make before the unlock.
    let mut h = open_locked(locked("reader", without(&[4])), Some("reader"));
    let id = h.state().views[0].id;
    assert!(add_rect(h.state_mut()), "commenting is allowed");
    h.run_steps(2);
    let before = h.state().session.get(id).unwrap().edit_generation();
    assert!(!h.state().session.get(id).unwrap().allows_modification());
    assert!(h.state_mut().execute("protect.unlock"));
    h.run_steps(2);
    for wrong in ["wrong", "reader"] {
        type_password(&mut h, wrong);
        h.get_by_label("Incorrect password. Try again.");
        let doc = h.state().session.get(id).unwrap();
        assert!(!doc.allows_modification() && !doc.security_summary().unwrap().owner, "{wrong} lifts nothing");
        assert!(h.state().unlock_prompt.is_some(), "{wrong}: the prompt stays");
    }
    type_password(&mut h, "owner");
    assert!(h.state().unlock_prompt.is_none());
    let doc = h.state().session.get(id).unwrap();
    assert!(doc.allows_modification() && doc.security_summary().unwrap().owner, "the rights are lifted");
    assert_eq!(doc.edit_generation(), before, "unlocking is no edit and no undo step");
    assert_eq!(doc.info.annotations.iter().filter(|a| a.subtype == "Square").count(), 1, "the edit made before is kept");
    assert!(h.query_by_label_contains(NOTICE).is_none(), "the notice is gone");
    // Saving keeps the security: the file still has both passwords and the same /P.
    let bytes = h.state().session.save_bytes(id).unwrap();
    let mut s = pdfcraft_engine::Session::new();
    let as_user = s.open("u.pdf", None, bytes.clone(), Some("reader")).unwrap();
    assert!(!s.get(as_user).unwrap().allows_modification(), "the user password still opens it restricted");
    let as_owner = s.open("o.pdf", None, bytes, Some("owner")).unwrap();
    assert!(s.get(as_owner).unwrap().allows_modification());
}

/// An encrypted document saved after an edit stays encrypted, with the same permissions and the
/// same passwords.
#[test]
fn an_encrypted_document_saved_after_an_edit_keeps_its_security() {
    let bits = without(&[4, 5]);
    let mut h = open_locked(locked("reader", bits), Some("reader"));
    let id = h.state().views[0].id;
    assert!(add_rect(h.state_mut()));
    h.run_steps(2);
    let bytes = h.state().session.save_bytes(id).unwrap();
    let mut s = pdfcraft_engine::Session::new();
    assert!(s.open("none.pdf", None, bytes.clone(), None).is_err(), "still needs a password");
    assert!(s.open("wrong.pdf", None, bytes.clone(), Some("wrong")).is_err());
    let user = s.open("user.pdf", None, bytes.clone(), Some("reader")).unwrap();
    let doc = s.get(user).unwrap();
    assert!(doc.info.encrypted);
    let p = doc.permissions().unwrap();
    assert!(!p.owner && !p.modify() && !p.copy() && p.print() && p.annotate() && p.assemble(), "the same permissions");
    assert_eq!(doc.info.annotations.iter().filter(|a| a.subtype == "Square").count(), 1, "and the edit");
    let owner = s.open("owner.pdf", None, bytes, Some("owner")).unwrap();
    assert!(s.get(owner).unwrap().permissions().unwrap().owner, "the owner password still opens it as owner");
}
