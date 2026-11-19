use crate::cli::exec::Exec;

/// Launch Node with a V8 startup snapshot.
///
/// `mise vsl -- --snapshot-blob snapshot.blob app.js` resolves Node from the
/// active Mise configuration and forwards the arguments directly to it.
#[derive(Debug, usage_rs::Args)]
#[usage(
    verbatim_doc_comment,
    example(
        "mise vsl -- --snapshot-blob snapshot.blob app.js",
        help = "Run app.js from a V8 snapshot"
    )
)]
pub(crate) struct Vsl {
    /// Arguments passed directly to Node after `--`
    #[usage(double_dash = "required", required = true)]
    args: Vec<String>,
}

impl Vsl {
    pub(crate) async fn run(self) -> eyre::Result<()> {
        let command = std::iter::once("node".to_string())
            .chain(self.args)
            .collect::<Vec<_>>();
        Exec {
            tool: vec![],
            command: Some(command),
            c: None,
            jobs: None,
            allow_env: vec![],
            allow_net: vec![],
            allow_read: vec![],
            allow_unix: vec![],
            allow_write: vec![],
            deny_all: false,
            deny_env: false,
            deny_net: false,
            deny_read: false,
            deny_unix: false,
            deny_write: false,
            fresh_env: false,
            no_deps: true,
            raw: false,
            secrets: vec![],
            secrets_all: false,
        }
        .run()
        .await
    }
}
