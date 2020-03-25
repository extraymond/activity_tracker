use afterglow::prelude::*;
use afterglow_router::route_to;

/// Total view if the page
#[derive(Default)]
pub struct View {
    pub head: HeadView,
    pub body: BodyView,
    pub footer: FooterView,
    pub opt_panel: OptionPanel,
}

/// visual element of the header, used to navigate, and trigger system wide events.
#[derive(Default)]
pub struct HeadView;

impl Renderer for HeadView {
    type Target = super::model::HeadContainer;
    type Data = super::model::Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        dodrio!(bump,
            <nav class="top-panel navbar">
                <div class="navbar-brand">
                    <a class="navbar-item jump" onclick={ route_to("") }>
                        <span class="icon is-medium">
                            <i class="mdi mdi-home "></i>
                        </span>
                        <span class="heading" style="margin-bottom: 0px">"Home"</span>
                    </a>
                    // <a class="button" class="navbar-burger">
                    //     <span></span>
                    //     <span></span>
                    //     <span></span>
                    // </a>
                </div>
                // <div class="navbar-menu">
                //     <div class="navbar-start">
                //         <div class="navbar-item">
                //             <div class="tag">"head!"</div>
                //         </div>
                //         <div class="navbar-item">
                //         </div>
                //     </div>
                //     <div class="navbar-end"></div>
                // </div>
            </nav>
        )
    }
}

/// visual element of the body, used to display content, and engage in user activities.
#[derive(Default)]
pub struct BodyView;


impl Renderer for BodyView {
    type Target = Container<crate::elements::dashboard::model::Model>;
    type Data = super::model::Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        dodrio!(bump, 
            <div class="body-section">
                    { target.render(ctx) }
            </div>)
    }
}

/// visiual element of the footer area, used to provide control and panel to assist user activiies.
#[derive(Default)]
pub struct FooterView;


impl Renderer for FooterView {
    type Target = super::model::FooterContainer;
    type Data = super::model::Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        dodrio!(bump,
                <div class="bottom-panel field has-addons">
                    <p class="control">
                        <button
                            onclick={consume(|_| { super::model::Event::RequestNew }, &sender)}
                            class="button is-dark is-outlined">
                                "add new event"
                        </button>
                        </p>
                    <p class="control">
                        <button
                            onclick={consume(|_| { super::model::Event::RequestRemove }, &sender)}
                            class="button is-dark is-outlined">
                                "remove selected"
                        </button>
                    </p>

                </div>
        )
    }
}

#[derive(Default)]
pub struct OptionPanel;
impl Renderer for OptionPanel {
    type Target = ();
    type Data = super::model::Model;

    fn view<'a>(&self, target: &Self::Target, ctx: &mut RenderContext<'a>, sender: &MessageSender<Self::Data>) -> Node<'a> {

        let bump = ctx.bump;
        dodrio!(bump, 
            <div class="opt_panel">
                <div class="field">
                    <label class="label">"filter"</label>
                    <div class="control is-expanded">
                        <div onclick={ consume(|_| { super::model::Event::RequestCleanup }, &sender) } class="button is-outlined is-fullwidth">"clean finished"</div>
                    </div>
                    
                </div>
            </div>
        )

    }

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

        dodrio!(bump,
            <div class="home is-unselectable">
                { self.head.view(&target.head, ctx, &sender) }
                { self.body.view(&target.body, ctx, &sender) }
                { self.footer.view(&target.footer, ctx, &sender) }
                { self.opt_panel.view(&(), ctx, &sender) }
            </div>
        )
    }
}
