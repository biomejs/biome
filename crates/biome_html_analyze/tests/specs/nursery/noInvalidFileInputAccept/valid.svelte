<!-- should not generate diagnostics -->
<input type="file" accept={allowedTypes} />
<input type={fileType} accept="image/jpg" />
<input type="file" accept="image/jpg" accept={allowedTypes} />
<input type="file" accept="image/jpg" {...props} />
