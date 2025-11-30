use zed_extension_api as zed;

struct CsoundExtension;

impl zed::Extension for CsoundExtension {
    fn new() -> Self
        where
            Self: Sized {
                Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree
    ) -> zed::Result<zed::Command> {


        eprintln!("Try sto start server!");
        let lsp_path = "/Users/pm/AcaHub/Coding/tree-sitter-csound/csound-lsp/target/release/csound-lsp";

        Ok(zed::Command {
            command: lsp_path.to_string(),
            args: vec![],
            env: Default::default()
        })
    }

}

zed::register_extension!(CsoundExtension);
