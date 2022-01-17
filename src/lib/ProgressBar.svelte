<div class="progress-bar">
  <div class="moving-bar">
    <div class="inner-bar" />
  </div>
</div>

<style lang="sass">
  .progress-bar
    position: relative
    width: 120px
    overflow: hidden
    height: 2px
    background-color: hsla(230, 100%, 75%, 0.2)
  .moving-bar
    width: 100%
    height: 100%
    position: absolute
    animation: load 2s linear infinite
  .inner-bar
    width: 100%
    height: 100%
    opacity: 0.7
    background: linear-gradient(45deg, hsl(215, 100%, 80%), hsl(172, 100%, 80%))
    animation: size 2s linear infinite

  @keyframes load
    0%
      transform: translateX(-75%)
    50%
      transform: translateX(0%)
    100%
      transform: translateX(75%)
  @keyframes size
    0%
      transform: scaleX(0.1)
      animation-timing-function: ease-out
    50%
      transform: scaleX(0.5)
      animation-timing-function: ease-in
    100%
      transform: scaleX(0.1)
</style>
