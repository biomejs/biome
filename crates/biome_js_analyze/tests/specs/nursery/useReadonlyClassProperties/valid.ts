class ValidContainer {
  // Public members might be modified externally
  public publicMember: boolean;

  // Protected members might be modified by child classes
  protected protectedMember: number;

  // This is modified later on by the class
  private modifiedLater = 'unchanged';

  public mutate() {
    this.modifiedLater = 'mutated';
  }

  // This is modified later on by the class
  #modifiedLaterPrivateField = 'unchanged';

  public mutatePrivateField() {
    this.#modifiedLaterPrivateField = 'mutated';
  }
}
