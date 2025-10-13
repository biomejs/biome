/* should not generate diagnostics */

// Valid: TypeScript file with proper props usage
interface Props {
  count: number;
  msg: string;
}

export default {
  setup(props: Props) {
    return () => props.count + props.msg;
  }
}
