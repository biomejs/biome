use biome_languages::JsFileSource;
use biome_languages::javascript::{
    JsEmbeddingKind, Language, LanguageVariant, LanguageVersion, ModuleKind, SvelteEmbeddingKind,
    SvelteFileKind,
};
use boa_engine::object::ObjectInitializer;
use boa_engine::property::Attribute;
use boa_engine::{Context, JsString, JsValue, js_string};
use camino::Utf8Path;

pub(crate) fn create_rule_context(
    path: &Utf8Path,
    source_type: JsFileSource,
    context: &mut Context,
) -> JsValue {
    let language = match source_type.language() {
        Language::JavaScript => ObjectInitializer::new(context)
            .property(
                js_string!("kind"),
                js_string!("javascript"),
                Attribute::ENUMERABLE,
            )
            .build(),
        Language::TypeScript { definition_file } => ObjectInitializer::new(context)
            .property(
                js_string!("kind"),
                js_string!("typescript"),
                Attribute::ENUMERABLE,
            )
            .property(
                js_string!("definitionFile"),
                definition_file,
                Attribute::ENUMERABLE,
            )
            .build(),
    };
    let variant = match source_type.variant() {
        LanguageVariant::Standard => js_string!("standard"),
        LanguageVariant::StandardRestricted => js_string!("standardRestricted"),
        LanguageVariant::Jsx => js_string!("jsx"),
    };
    let module_kind = match source_type.module_kind() {
        ModuleKind::Module => js_string!("module"),
        ModuleKind::Script => js_string!("script"),
    };
    let version = match source_type.version() {
        LanguageVersion::ES2022 => js_string!("es2022"),
        LanguageVersion::ESNext => js_string!("esNext"),
    };
    let embedding_kind = embedding_kind(source_type.as_embedding_kind(), context);
    let source_type = ObjectInitializer::new(context)
        .property(js_string!("language"), language, Attribute::ENUMERABLE)
        .property(js_string!("variant"), variant, Attribute::ENUMERABLE)
        .property(js_string!("moduleKind"), module_kind, Attribute::ENUMERABLE)
        .property(js_string!("version"), version, Attribute::ENUMERABLE)
        .property(
            js_string!("embeddingKind"),
            embedding_kind,
            Attribute::ENUMERABLE,
        )
        .build();
    ObjectInitializer::new(context)
        .property(
            js_string!("filePath"),
            JsString::from(path.as_str()),
            Attribute::ENUMERABLE,
        )
        .property(js_string!("sourceType"), source_type, Attribute::ENUMERABLE)
        .build()
        .into()
}

fn embedding_kind(kind: &JsEmbeddingKind, context: &mut Context) -> JsValue {
    let mut object = ObjectInitializer::new(context);
    match kind {
        JsEmbeddingKind::None => {
            object.property(
                js_string!("kind"),
                js_string!("none"),
                Attribute::ENUMERABLE,
            );
        }
        JsEmbeddingKind::Astro {
            frontmatter,
            is_class_attribute,
        } => {
            object
                .property(
                    js_string!("kind"),
                    js_string!("astro"),
                    Attribute::ENUMERABLE,
                )
                .property(
                    js_string!("frontmatter"),
                    *frontmatter,
                    Attribute::ENUMERABLE,
                )
                .property(
                    js_string!("isClassAttribute"),
                    *is_class_attribute,
                    Attribute::ENUMERABLE,
                );
        }
        JsEmbeddingKind::Vue {
            setup,
            is_source,
            event_handler,
            ..
        } => {
            object
                .property(js_string!("kind"), js_string!("vue"), Attribute::ENUMERABLE)
                .property(js_string!("setup"), *setup, Attribute::ENUMERABLE)
                .property(js_string!("isSource"), *is_source, Attribute::ENUMERABLE)
                .property(
                    js_string!("eventHandler"),
                    *event_handler,
                    Attribute::ENUMERABLE,
                );
        }
        JsEmbeddingKind::Svelte {
            file_kind,
            embedding_kind,
        } => {
            let file_kind = match file_kind {
                SvelteFileKind::Component => js_string!("component"),
                SvelteFileKind::SourceModule => js_string!("sourceModule"),
            };
            let embedding_kind = match embedding_kind {
                SvelteEmbeddingKind::Source => js_string!("source"),
                SvelteEmbeddingKind::Expression => js_string!("expression"),
                SvelteEmbeddingKind::SnippetSignature => js_string!("snippetSignature"),
                SvelteEmbeddingKind::LegacyConst => js_string!("legacyConst"),
                SvelteEmbeddingKind::Declaration => js_string!("declaration"),
            };
            object
                .property(
                    js_string!("kind"),
                    js_string!("svelte"),
                    Attribute::ENUMERABLE,
                )
                .property(js_string!("fileKind"), file_kind, Attribute::ENUMERABLE)
                .property(
                    js_string!("embeddingKind"),
                    embedding_kind,
                    Attribute::ENUMERABLE,
                );
        }
    }
    object.build().into()
}
