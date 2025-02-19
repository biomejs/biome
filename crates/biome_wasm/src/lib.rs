use js_sys::Error;
use wasm_bindgen::prelude::*;

use biome_fs::MemoryFileSystem;
use biome_service::workspace::{
    self, ChangeFileParams, CloseFileParams, FixFileParams, FormatFileParams, FormatOnTypeParams,
    FormatRangeParams, GetControlFlowGraphParams, GetFileContentParams, GetFormatterIRParams,
    GetSyntaxTreeParams, OpenProjectParams, PullActionsParams, PullDiagnosticsParams, RenameParams,
    UpdateSettingsParams,
};
use biome_service::workspace::{OpenFileParams, SupportsFeatureParams};

mod utils;

pub use crate::utils::DiagnosticPrinter;
use crate::utils::{into_error, set_panic_hook};

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

include!(concat!(env!("OUT_DIR"), "/ts_types.rs"));

#[wasm_bindgen]
pub struct Workspace {
    inner: Box<dyn workspace::Workspace>,
}

#[wasm_bindgen]
impl Workspace {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Workspace {
        Workspace {
            inner: workspace::server(Box::new(MemoryFileSystem::default())),
        }
    }

    #[wasm_bindgen(js_name = fileFeatures)]
    pub fn file_features(
        &self,
        params: ISupportsFeatureParams,
    ) -> Result<IFileFeaturesResult, Error> {
        let params: SupportsFeatureParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.file_features(params).map_err(into_error)?;
        to_value(&result)
            .map(IFileFeaturesResult::from)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = updateSettings)]
    pub fn update_settings(
        &self,
        params: IUpdateSettingsParams,
    ) -> Result<IUpdateSettingsResult, Error> {
        let params: UpdateSettingsParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.update_settings(params).map_err(into_error)?;
        to_value(&result)
            .map(IUpdateSettingsResult::from)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = openProject)]
    pub fn open_project(&self, params: IOpenProjectParams) -> Result<IProjectKey, Error> {
        let params: OpenProjectParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.open_project(params).map_err(into_error)?;

        to_value(&result).map(IProjectKey::from).map_err(into_error)
    }

    #[wasm_bindgen(js_name = openFile)]
    pub fn open_file(&self, params: IOpenFileParams) -> Result<(), Error> {
        let params: OpenFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.open_file(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = getFileContent)]
    pub fn get_file_content(&self, params: IGetFileContentParams) -> Result<String, Error> {
        let params: GetFileContentParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.get_file_content(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = getSyntaxTree)]
    pub fn get_syntax_tree(
        &self,
        params: IGetSyntaxTreeParams,
    ) -> Result<IGetSyntaxTreeResult, Error> {
        let params: GetSyntaxTreeParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.get_syntax_tree(params).map_err(into_error)?;
        to_value(&result)
            .map(IGetSyntaxTreeResult::from)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = getControlFlowGraph)]
    pub fn get_control_flow_graph(
        &self,
        params: IGetControlFlowGraphParams,
    ) -> Result<String, Error> {
        let params: GetControlFlowGraphParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner
            .get_control_flow_graph(params)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = getFormatterIr)]
    pub fn get_formatter_ir(&self, params: IGetFormatterIRParams) -> Result<String, Error> {
        let params: GetFormatterIRParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.get_formatter_ir(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = changeFile)]
    pub fn change_file(&self, params: IChangeFileParams) -> Result<(), Error> {
        let params: ChangeFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.change_file(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = closeFile)]
    pub fn close_file(&self, params: ICloseFileParams) -> Result<(), Error> {
        let params: CloseFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        self.inner.close_file(params).map_err(into_error)
    }

    #[wasm_bindgen(js_name = pullDiagnostics)]
    pub fn pull_diagnostics(
        &self,
        params: IPullDiagnosticsParams,
    ) -> Result<IPullDiagnosticsResult, Error> {
        let params: PullDiagnosticsParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.pull_diagnostics(params).map_err(into_error)?;
        to_value(&result)
            .map(IPullDiagnosticsResult::from)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = pullActions)]
    pub fn pull_actions(&self, params: IPullActionsParams) -> Result<IPullActionsResult, Error> {
        let params: PullActionsParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.pull_actions(params).map_err(into_error)?;
        to_value(&result)
            .map(IPullActionsResult::from)
            .map_err(into_error)
    }

    #[wasm_bindgen(js_name = formatFile)]
    pub fn format_file(&self, params: IFormatFileParams) -> Result<JsValue, Error> {
        let params: FormatFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.format_file(params).map_err(into_error)?;
        to_value(&result).map_err(into_error)
    }

    #[wasm_bindgen(js_name = formatRange)]
    pub fn format_range(&self, params: IFormatRangeParams) -> Result<JsValue, Error> {
        let params: FormatRangeParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.format_range(params).map_err(into_error)?;
        to_value(&result).map_err(into_error)
    }

    #[wasm_bindgen(js_name = formatOnType)]
    pub fn format_on_type(&self, params: IFormatOnTypeParams) -> Result<JsValue, Error> {
        let params: FormatOnTypeParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.format_on_type(params).map_err(into_error)?;
        to_value(&result).map_err(into_error)
    }

    #[wasm_bindgen(js_name = fixFile)]
    pub fn fix_file(&self, params: IFixFileParams) -> Result<IFixFileResult, Error> {
        let params: FixFileParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.fix_file(params).map_err(into_error)?;
        to_value(&result)
            .map(IFixFileResult::from)
            .map_err(into_error)
    }

    pub fn rename(&self, params: IRenameParams) -> Result<IRenameResult, Error> {
        let params: RenameParams =
            serde_wasm_bindgen::from_value(params.into()).map_err(into_error)?;
        let result = self.inner.rename(params).map_err(into_error)?;
        to_value(&result)
            .map(IRenameResult::from)
            .map_err(into_error)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

fn to_value<T: serde::ser::Serialize + ?Sized>(
    value: &T,
) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true))
}
