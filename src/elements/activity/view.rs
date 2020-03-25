use super::model;
use afterglow::prelude::*;

#[derive(Default)]
pub struct View;

impl View {
    const STYLE: &'static str = r#"
            display: grid;
            column-gap: 0.5rem;
            grid-template-columns: 2fr 2fr 8fr 2fr;
            grid-template-rows: 2rem 4rem;
            opacity 1;
            transition: all 1s;
        "#;
}

impl Renderer for View {
    type Target = super::model::Activity;
    type Data = super::model::Activity;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        let start = bf!(in bump, "{}", target.start.time().format("%H:%M:%S")).into_bump_str();
        // let now = bf!(in bump, "{}", target.now.time()).into_bump_str();
        let elasped = target.elasped.map(|elasped| {
            bf!(in bump, "{:02}:{:02}:{:02}", elasped.num_hours(), elasped.num_minutes(), elasped.num_seconds())
                .into_bump_str()
        }).or(Some( 
            "---"
        )).map(|val| text(val));
        
        let status = bf!(in bump, "{}", target.status).into_bump_str();

        let action_hint = match target.status {
            model::Status::Started => "start",
            model::Status::Paused => "continue",
            model::Status::Running => "pause",
            model::Status::Completed => "completed",
            _ => ""
        };
        let button_status = match target.status {
            model::Status::Completed => true,
            _ => false
        };
        let action_hint = bf!(in bump, "{}", action_hint).into_bump_str();
        let mut style = View::STYLE.to_string();
        if let super::model::Status::Exiting = target.status {
            style.push_str("\n opacity: 0;");
        }

        dodrio!(bump,
            <div class="box is-size-7 is-size-6-desktop" style={ style } >
                <p class="heading">"start"</p>
                <p class="heading">"elasped"</p>
                <p class="heading">"status"</p>
                <div></div>

                <div>{ text(start) }</div>
                <div>{ elasped  }</div>
                <div>{ text(status) }</div>
                <div style="height: 100%; display: flex; align-items: center">
                    <button onclick={consume(|e| {
                            e.stop_propagation();
                            model::Event::Clicked}, &sender)}
                        class="button"
                        disabled={ button_status }
                        style="width: 6rem"
                        >{ text(action_hint) }
                    </button>
                </div>
            </div>
        )
    }
}
