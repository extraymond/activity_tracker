use afterglow::prelude::*;

/// request/display permission to show notification.
#[derive(Default)]
pub struct Controller;
impl Renderer for Controller {
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
            <div class="box">
                <p class="heading">"A notification box"</p>
                { ViewStatus.view(&target.status, ctx, sender) }
                { ViewButton.view(&target.status, ctx, sender) }
            </div>
        )
    }
}

pub struct ViewStatus;
impl Renderer for ViewStatus {
    type Target = web_sys::NotificationPermission;
    type Data = super::model::Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        let status = match target {
            web_sys::NotificationPermission::Default => "site default",
            web_sys::NotificationPermission::Granted => "granted",
            web_sys::NotificationPermission::Denied => "denied",
            _ => "unknown",
        };
        let status = bf!(in bump, "{}", status).into_bump_str();

        dodrio!(bump, <div class="tag">{ text(status) }</div>)
    }
}

pub struct ViewButton;
impl Renderer for ViewButton {
    type Target = web_sys::NotificationPermission;
    type Data = super::model::Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;
        let button = match target {
            web_sys::NotificationPermission::Granted => None,
            web_sys::NotificationPermission::Denied => {
                Some(dodrio!(bump, <div class="tag">"notification denied"</div>))
            }
            _ => Some(
                dodrio!(bump, <a onclick={consume(|_| { super::model::Event::Request}, &sender) } class="button">"request permission"</a>),
            ),
        };

        dodrio!(bump, <div>{ button }</div>)
    }
}
