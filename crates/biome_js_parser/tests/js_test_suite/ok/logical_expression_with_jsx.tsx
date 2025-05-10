import React from 'react';
let b = 1;
let c = 2;

export function A() {
  const a = b >= 0 && b < c
  return (
    <C.D>
     {a}
    </C.D>
  )
}

// issue: https://github.com/biomejs/biome/issues/5876 
export default function UserWorkspaceTeamCellAvatarList() {
  const displayedIds = ['']
  const totalIds = 1
  const displayMoreIndicator: boolean = displayedIds.length > 0 && displayedIds.length < totalIds

  return (
    <React.Fragment>
      {displayMoreIndicator}
    </React.Fragment>
  )
}
