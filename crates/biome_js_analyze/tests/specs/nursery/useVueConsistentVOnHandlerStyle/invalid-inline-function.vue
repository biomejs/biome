<!-- should generate diagnostics -->
<template>
  <button @click="foo" />
  <button @click="foo()" />
  <button @click="foo($event)" />
  <button @click="count++" />
  <button @click="foo( )" />
  <button @click="{foo();foo();}" />
  <button @click="foo();foo();" />
  <button @click="foo();" />
  <button @click=" foo() " />
  <button @click="foo() /* comment */" />
  <button @click=foo() />
  <button @click="foo(), bar()" />
</template>
