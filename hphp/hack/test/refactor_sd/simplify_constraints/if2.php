<?hh

<<__SupportDynamicType>>
function g(): void {}

<<__SupportDynamicType>>
function f(): void {}

function h(): void {
  $b = true;
  $c = false;
  if ($b) {
    if ($c) {
      $x = g<>;
    } else {
      f<> upcast dynamic;
    }
  } else {
    if ($c) {
      $x = f<>;
    } else {
      f<> upcast dynamic;
    }
  }
  $b upcast dynamic;
}
