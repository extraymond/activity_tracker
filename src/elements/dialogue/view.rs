use afterglow::prelude::*;

#[derive(Default)]
pub struct View;
impl Renderer for View {
    type Target = super::model::Model;
    type Data = super::model::Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        let class = if target.active {
            "modal is-active"
        } else {
            "modal "
        };

        let style = format!("transition:opacity 0.5s; opacity: {}", target.visible);

        dodrio!(bump,
                <div class={ class } style= { style }>
                    <div class="modal-background"></div>
                    <div class="modal-content">
                        <div class="box">"hey you missed me?"</div>
                    </div>
                    <button onclick={consume(|_| { super::model::Event::CloseWithAnimation(500)}, &sender)} class="modal-close is-large">"close"</button>
                </div>
        )
    }
}
