<script lang="ts">
  export let disabled = false
</script>

<button class:disabled on:click>
  <slot />
</button>

<style lang="sass">
  $ease-out: cubic-bezier(0, 0, 0.2, 1)
  button
    font-family: inherit
    user-select: none
    outline: none
    box-sizing: border-box
    padding: 7px 28px
    font-size: 13px
    margin: 15px
    background-color: hsla(172, 80%, 75%, 0.1)
    border: 1px solid hsla(172, 100%, 80%, 0.3)
    color: hsla(172, 100%, 95%, 0.8)
    font-weight: 500
    transition: all 200ms ease-out
    &:focus
      border: 1px solid hsla(172, 100%, 80%, 0.5)
      box-shadow: 0px 0px 0px 2px hsla(172, 100%, 80%, 0.2)
    &:hover
      border: 1px solid hsla(172, 100%, 80%, 0.5)
      box-shadow: 0px 0px 0px 2px hsla(172, 100%, 80%, 0.2)
    &:active
      background-color: hsla(172, 80%, 75%, 0.15)
</style>
