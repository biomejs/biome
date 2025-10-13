// Invalid: TypeScript file with destructuring
interface Props {
  count: number;
  msg: string;
}

export default {
  setup(props: Props) {
    const { count, msg } = props;
    return { count, msg };
  }
}
