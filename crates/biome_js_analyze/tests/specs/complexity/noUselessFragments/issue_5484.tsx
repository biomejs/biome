export const ChatSettings: Component<ChatProps> = (props) => {
  return (
    <div {...stylex.attrs(styles.base)}>
      <>system prompt:{' '}</>
      <BlockArray blockArray={props.chat.state.system_prompt} processMarkdown={false} />
      <ModelSelector chat={props.chat} />
    </div>
  )
}
