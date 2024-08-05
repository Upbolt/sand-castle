# Sand Castle
High level Rust library intended for integration with web frameworks to facilitate 3D scenes.

## Officially Supported Frameworks
> ### [Leptos](https://github.com/leptos-rs/leptos)
> ```rs
> #[component]
> fn App() -> impl IntoView {
>   let yaw = RwSignal::new(-89.555);
>   let pitch = RwSignal::new(-19.25);
>
>   view! {
>     <h1 attr:style="margin-bottom: 0.125rem">"sand-castle leptos"</h1>
>
>     <Canvas attr:style="display: block">
>       <Scene color=Vec4::new(0.1, 0.1, 0.1, 1.0)>
>         <PerspectiveCamera
>           aspect_ratio=300.0 / 150.0
>           position=Vec3::new(0.0, 1.5, 4.0)
>           yaw=yaw
>           pitch=pitch
>         />
>
>         <Mesh>
>           <Cuboid />
>           <BasicMaterial />
>         </Mesh>
>       </Scene>
>     </Canvas>
>
>     <div>
>       <h2 style="margin-bottom: 0.25rem">"camera"</h2>
>
>       <div>
>         <button on:click=move |_| yaw.update(|yaw| { *yaw += 0.1; })>"+"</button>
>         <button on:click=move |_| yaw.update(|yaw| { *yaw -= 0.1; })>"-"</button>
>         <span>{move || format!("yaw: {}", yaw.get())}</span>
>       </div>
>
>       <div>
>         <button on:click=move |_| pitch.update(|pitch| { *pitch += 0.1; })>"+"</button>
>         <button on:click=move |_| pitch.update(|pitch| { *pitch -= 0.1; })>"-"</button>
>         <span>{move || format!("pitch: {}", pitch.get())}</span>
>       </div>
>     </div>
>   }
> }
> ```
