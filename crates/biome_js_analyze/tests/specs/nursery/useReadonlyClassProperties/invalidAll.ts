class InvalidAllContainer {
  // These member variables could be marked as readonly
  private neverModifiedMember = true;
  private onlyModifiedInConstructor: number;
  #neverModifiedPrivateField = 3;
	public publicNeverModifiedMember = true;
  public publicOnlyModifiedInConstructor: number;
	protected protectedNeverModifiedMember = true;
  protected protectedOnlyModifiedInConstructor: number;

  public constructor(
    onlyModifiedInConstructor: number,
    // These member variables could be marked as readonly
    private neverModifiedParameter: string,
		public publicNeverModifiedParameter: string,
		protected protectedNeverModifiedParameter: string,
  ) {
    this.onlyModifiedInConstructor = onlyModifiedInConstructor;
		this.publicOnlyModifiedInConstructor = onlyModifiedInConstructor;
		this.protectedOnlyModifiedInConstructor = onlyModifiedInConstructor;
  }
}
