// 5-len regex
z.string().min(2).max(200).regex(/[a-z]/gm, {
	message: 'Only English alphabet symbols and hyphen allowed',
})

// <5-len regex
z.string().min(2).max(200).regex(/\w+/gm, {
	message: 'Only English alphabet symbols and hyphen allowed',
})


// >5-len regex
z.string().min(2).max(200).regex(/^[a-zA-Z\-]+$/gm, {
	message: 'Only English alphabet symbols and hyphen allowed',
})

const a = {
  locales: z.record(
    localeKeySchema,
    z.string().min(2).regex(/^[a-zA-Z\-]+$/gm, {
      message: 'Only English alphabet symbols and hyphen allowed',
    })
  ),
}
