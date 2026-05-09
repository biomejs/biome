<template>
	<Transition name="fade">
		<BaseLoader v-if="isLoading" />
	</Transition>
</template>

<template>
	<KeepAlive>
		<UserPanel />
	</KeepAlive>
</template>

<template>
	<Teleport to="body">
		<UserPanel />
	</Teleport>
</template>

<template>
	<UForm>
		<BaseLoader v-if="isLoading" />
	</UForm>
</template>

<template>
	<Layout.Header>
		<UserAvatar />
	</Layout.Header>
</template>

<template>
	<my-widget>
		<inner-widget />
	</my-widget>
</template>

<template>
	<span>
		<em>label</em>
	</span>
</template>

<template>
	<Transition name="fade"><BaseLoader v-if="isLoading" /></Transition>
</template>

<template>
	<UForm>
		Submit
	</UForm>
</template>

<template>
	<div>
		<span>label</span>
	</div>
</template>

<template>
	<UForm>
		<BaseLoader />
		<UserPanel />
	</UForm>
</template>

<template>
	<UForm>
		Before
		<BaseLoader />
		After
	</UForm>
</template>

<template>
	<template #default>
		<UserPanel />
	</template>
</template>
