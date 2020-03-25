use afterglow::prelude::*;
#[derive(Default)]
pub struct View;

impl View {
    const STYLE: &'static str = r#"
        min-height: 100%; 
        max-height: 100%; 
        min-width: 100%;
        max-width: 100%;
    "#;

    const TABLE_STYLE: &'static str = r#"
    display: grid;
    grid-template-columns: 1fr;
    row-gap: 1rem;
    padding: 1rem;
    "#;
}

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

        let activites = target
            .activities
            .iter()
            .enumerate()
            .map(|(idx, activity)| {
                let selected = if let Some(id) = target.selected {
                    if id == idx {
                        "transition: margin-left 0.5s; margin-left: 0.5rem; border-right: 0.5rem solid #845ef7"
                    } else {
                        "transition: margin-left 0.5s"
                    }
                } else {
                    "transition: margin-left 0.5s"
                };
                dodrio!(bump,
                    <div style={ selected } onclick={ consume(move |_| { super::model::Event::Selected(idx) }, &sender.clone())}>{ activity.render(ctx) }</div>
                )
            });

        dodrio!(bump,
            <div style={ View::STYLE }>
                <div style={ View::TABLE_STYLE }>
                    { activites }
                </div>
            </div>
        )
    }
}
