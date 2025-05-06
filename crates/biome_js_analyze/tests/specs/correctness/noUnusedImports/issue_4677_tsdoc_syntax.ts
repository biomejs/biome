// See https://github.com/biomejs/biome/issues/4677

import type SyntaxNormal from "mod";
import type SyntaxWithDotMember from "mod";
import type SyntaxWithHashMember from "mod";
import type SyntaxWithDotMemberAndPipeDescription from "mod";
import type SyntaxWithHashMemberAndPipeDescription from "mod";
import type SyntaxWithDotMemberAnSpaceDescription from "mod";
import type SyntaxWithHashMemberAndSpaceDescription from "mod";

/**
 * {@link SyntaxNormal}
 */
function testSyntaxNormal() { }

/**
 * {@link SyntaxWithDotMember.member}
 */
function testSyntaxWithDotMember() { }

/**
 * {@link SyntaxWithHashMember#member}
 */
function testSyntaxWithHashMember() { }

/**
 * {@link SyntaxWithDotMemberAndPipeDescription.member|Description}
 */
function testSyntaxWithDotMemberAndPipeDescription() { }

/**
 * {@link SyntaxWithHashMemberAndPipeDescription#member|Description}
 */
function testSyntaxWithHashMemberAndPipeDescription() { }

/**
 * {@link SyntaxWithDotMemberAnSpaceDescription.member Description}
 */
function testSyntaxWithDotMemberAnSpaceDescription() { }

/**
 * {@link SyntaxWithHashMemberAndSpaceDescription#member Description}
 */
function testSyntaxWithHashMemberAndSpaceDescription() { }

