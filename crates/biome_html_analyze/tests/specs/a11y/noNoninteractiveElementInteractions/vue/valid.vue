<!-- should not generate diagnostics -->
<input @click="()=> void 0" />
<input type="button" @click="()=> void 0" />
<input type="checkbox" @click="()=> void 0" />
<input type="color" @click="()=> void 0" />

<!-- Interactive element -->
<a @click="()=> void 0"></a>
<area @click="() => { }" />
<button @click="()=> void 0"></button>
<menuitem @click="() => { }">
</menuitem>
<option @click="()=> void 0" class="foo"></option>
<select @click="()=> void 0" class="foo"></select>
<textarea @click="()=> void 0" class="foo"></textarea>
<tr @click="() => { }"></tr>

<!-- Element attributed with an interactive role -->
<div role="button" @click="() => { }"></div>
<div role="button" v-on:click="() => { }"></div>
<div role="checkbox" @click="() => { }"></div>
<div role="combobox" @click="() => { }"></div>
<div role="gridcell" @click="() => { }"></div>
<div role="link" @click="() => { }"></div>
<div role="listbox" @click="() => { }"></div>
<div role="menuitem" @click="() => { }"></div>
<div role="menuitemcheckbox" @click="() => { }"></div>
<div role="menuitemradio" @click="() => { }"></div>
<div role="option" @click="() => { }"></div>
<div role="radio" @click="() => { }"></div>
<div role="row" @click="() => { }"></div>
<div role="scrollbar" @click="() => { }"></div>
<div role="searchbox" @click="() => { }"></div>
<div role="slider" @click="() => { }"></div>
<div role="spinbutton" @click="() => { }"></div>
<div role="switch" @click="() => { }"></div>
<div role="tab" @click="() => { }"></div>
<div role="textbox" @click="() => { }"></div>

<!-- Presentation role -->
<div role="presentation" @click="() => { }"></div>

<div @click="() => { }" aria-hidden="true"></div>
