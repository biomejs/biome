const this_is_a_very_long_function_name_so_need_to_break_a_new_line_here = (
  id
) => {
  return id;
};

type InstanceID = string;
type MaybeCardWithAttachment = string;
function outerFunctionToForceIndent() {
    const cardWithAttachment: (id: InstanceID) => MaybeCardWithAttachment = (
        id
    ) => {
        return `${id}test`;
    };
}
