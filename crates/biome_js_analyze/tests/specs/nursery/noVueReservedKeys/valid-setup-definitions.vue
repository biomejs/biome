<script setup>
// Imports and variable definitions of SFC should not be flagged as reserved keys.
import {$el} from 'module1';
import {default as $slots} from 'module2';
import {x as $watch} from 'module3';
import {$watch as safe} from 'module4';
import $parent from 'module5';
import $destroy, {y as $emit} from 'module6';
const $set = 123;
let $data = 'test';
</script>
