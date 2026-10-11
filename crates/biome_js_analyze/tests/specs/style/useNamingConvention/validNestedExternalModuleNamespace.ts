/* should not generate diagnostics */
declare module "myExternalModule" {
    namespace Outer {
        namespace my_NAMESPACE {}
        interface my_INTERFACE {}
    }
}
