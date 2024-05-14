function test<
	longlonglonglonglonglonglongT1,
	longlonglonglonglonglonglongT2,
	longlonglonglonglonglonglongT3
	>(
	longlonglonglonglonglonglongItem1,
	longlonglonglonglonglonglongItem2,
	longlonglonglonglonglonglongItem3
) {}
const test1 = <
	longlonglonglonglonglonglongT1,
	longlonglonglonglonglonglongT2,
	longlonglonglonglonglonglongT3
	>(
	longlonglonglonglonglonglongItem1,
	longlonglonglonglonglonglongItem2,
	longlonglonglonglonglonglongItem3
) => {};
test<
	longlonglonglonglonglonglongT1,
	longlonglonglonglonglonglongT2,
	longlonglonglonglonglonglongT3
	>(
	longlonglonglonglonglonglongItem1,
	longlonglonglonglonglonglongItem2,
	longlonglonglonglonglonglongItem3,
);

this.test(	longlonglonglonglonglonglongT1,
	longlonglonglonglonglonglongT2,
	longlonglonglonglonglonglongT3);

connect(
	mapStateToPropsmapStateToProps,
	mapDispatchToPropsmapDispatchToProps,
	mergePropsmergeProps,
)(Component)
