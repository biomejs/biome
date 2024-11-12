class ValidAllContainer {
  public readonly publicMember: boolean;

  protected readonly protectedMember: number;

  private modifiedLater = 'unchanged';

  public mutate() {
    this.modifiedLater = 'mutated';
  }

  #modifiedLaterPrivateField = 'unchanged';

  public mutatePrivateField() {
    this.#modifiedLaterPrivateField = 'mutated';
  }
}
