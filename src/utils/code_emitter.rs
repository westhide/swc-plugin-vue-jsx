use swc_core::{
    common::{sync::Lrc, SourceMap},
    ecma::codegen::{text_writer::JsWriter, Config, Emitter, Node},
};

pub struct CodeEmitter;

impl CodeEmitter {
    pub fn emit<N: Node>(node: &N) -> String {
        let cm = Lrc::<SourceMap>::default();
        let mut code_buf: Vec<u8> = Vec::new();

        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "\n", &mut code_buf, None),
        };

        node.emit_with(&mut emitter).unwrap();

        String::from_utf8(code_buf).unwrap()
    }
}
