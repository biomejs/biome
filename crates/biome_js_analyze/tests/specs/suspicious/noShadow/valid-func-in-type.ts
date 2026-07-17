/** should not generate diagnostics */
type Props = {
  contact: string;
  onChange?: (contact: string) => void;
};
const { contact = "", onChange = () => {} }: Props = {} as Props;
