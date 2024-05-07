use leptos::*;

#[component]
pub fn Canvas(#[prop(optional)] node_ref: NodeRef<html::Canvas>) -> impl IntoView {
  Effect::new(move |_| {});

  view! {
    <canvas node_ref=node_ref />
  }
}
