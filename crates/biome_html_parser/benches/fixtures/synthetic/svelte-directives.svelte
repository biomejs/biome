<script>
  let value = '';
  let checked = false;
  let selected = '';
  let files = null;
  let group = [];
  let innerWidth = 0;
  let innerHeight = 0;
  let online = true;
  let visible = true;
  let fullscreenElement = null;
  let className = 'default';
  let color = '#ff0000';
  let fontSize = '16px';
  let opacity = 1;
  let transform = 'none';
  let mounted = false;
  let hovered = false;
  let focused = false;
  let validated = false;
  let inputRef;
  let selectRef;
  let boxRef;
  let boxWidth;
  let boxHeight;
  let offsetWidth;
  let offsetHeight;
  let scrollY;
  let isIndeterminate = false;
  let currentTime = 0;
  let duration = 0;
  let paused = true;
  let volume = 1;
  let muted = false;
  let playbackRate = 1;
  let active = false;
  let items = [
    { id: 1, name: 'A', visible: false },
    { id: 2, name: 'B', visible: false },
    { id: 3, name: 'C', visible: false },
  ];

  function handleInput() {}
  function handleChange() {}
  function handleKeyPress() {}
  function handleFirstKey() {}
  function handleKeyUp() {}
  function action1(node) {}
  function action2(node, param) {}
  function action3(node, config) {}
  function toggleCheck() {}
  function handleSelect() {}
  function handleFileSelect() {}
  function handleGroupChange() {}
  function fade(node, params) {}
  function fly(node, params) {}
  function slide(node, params) {}
  function flip(node, params) {}
  function tooltip(node, params) {}
  function clickOutside(node, handler) {}
  function longpress(node, params) {}
  function handleBoxClick() {}
  function handleDoubleClick() {}
  function scale(node, params) {}
  function blur(node, params) {}
  function inView(node, params) {}
  function videoAction(node) {}
  function audioEnhancer(node) {}
  function handleTimeUpdate() {}
  function handleAudioPlay() {}
  function handleAudioPause() {}
</script>

<svelte:window bind:innerWidth bind:innerHeight bind:online />
<svelte:document bind:visibilityState={visible} bind:fullscreenElement />
<svelte:body bind:scrollY />

<div class="directives-container">
  <input
    type="text"
    bind:value
    bind:this={inputRef}
    class:active={focused}
    class:invalid={!validated}
    style:color
    style:font-size={fontSize}
    style:opacity={`${opacity}%`}
    style:transform
    on:input={handleInput}
    on:change={handleChange}
    on:focus={() => focused = true}
    on:blur={() => focused = false}
    on:keypress|preventDefault|stopPropagation={handleKeyPress}
    on:keydown|once={handleFirstKey}
    on:keyup|self={handleKeyUp}
    use:action1
    use:action2={actionParam}
    use:action3|param1|param2={actionConfig}
  />

  <input
    type="checkbox"
    bind:checked
    bind:indeterminate={isIndeterminate}
    on:click|preventDefault={toggleCheck}
    class:checked
    class:indeterminate={isIndeterminate}
  />

  <select
    bind:value={selected}
    bind:this={selectRef}
    on:change|stopPropagation={handleSelect}
    class:has-value={selected !== ''}
  >
    <option value="">Select...</option>
    <option value="a">Option A</option>
    <option value="b">Option B</option>
  </select>

  <input
    type="file"
    bind:files
    multiple
    accept="image/*"
    on:change|preventDefault={handleFileSelect}
  />

  {#each ['a', 'b', 'c', 'd', 'e'] as option}
    <label>
      <input
        type="checkbox"
        bind:group
        value={option}
        on:change|stopPropagation={handleGroupChange}
      />
      <span class:selected={group.includes(option)}>{option}</span>
    </label>
  {/each}

  <div
    class="animated-box"
    class:mounted
    class:hovered
    class:visible
    style:background-color={color}
    style:border-color={checked ? 'green' : 'red'}
    style:display={visible ? 'block' : 'none'}
    in:fade={{ duration: 300 }}
    out:fly={{ y: 20, duration: 200 }}
    transition:slide|local={{ duration: 400 }}
    animate:flip={{ duration: 250 }}
    use:tooltip={{ text: 'This is a tooltip' }}
    use:clickOutside={() => (active = false)}
    use:longpress|preventDefault={handleLongPress}
    on:mouseenter={() => hovered = true}
    on:mouseleave={() => hovered = false}
    on:click|self|stopPropagation={handleBoxClick}
    on:dblclick|preventDefault|stopPropagation={handleDoubleClick}
    bind:this={boxRef}
    bind:clientWidth={boxWidth}
    bind:clientHeight={boxHeight}
    bind:offsetWidth
    bind:offsetHeight
  >
    Content with many directives
  </div>

  <ul>
    {#each items as item, index (item.id)}
      <li
        class:even={index % 2 === 0}
        class:odd={index % 2 !== 0}
        class:first={index === 0}
        class:last={index === items.length - 1}
        style:animation-delay={`${index * 50}ms`}
        in:scale={{ start: 0.8, duration: 200 }}
        out:blur={{ duration: 150 }}
        animate:flip
        use:inView={{ threshold: 0.5 }}
        on:enterViewport={() => item.visible = true}
        on:leaveViewport={() => item.visible = false}
      >
        {item.name}
      </li>
    {/each}
  </ul>

  <video
    bind:currentTime
    bind:duration
    bind:paused
    bind:volume
    bind:muted
    bind:playbackRate
    on:play|once={() => console.log('started')}
    on:pause={() => console.log('paused')}
    on:ended={() => console.log('ended')}
    on:timeupdate|self={handleTimeUpdate}
    use:videoAction
  >
    <source src="video.mp4" type="video/mp4" />
  </video>

  <audio
    bind:currentTime
    bind:duration
    bind:paused
    bind:volume
    bind:muted
    on:play={handleAudioPlay}
    on:pause={handleAudioPause}
    use:audioEnhancer
  >
    <source src="audio.mp3" type="audio/mpeg" />
  </audio>
</div>
