pub mod hooks {
    use penrose::{
        core::{hooks::StateHook, State},
        x::XConn,
        x11rb::RustConn,
        Xid,
    };

    /// Create a startup hook by passing a list of programs to spawn
    pub fn startup_programs<X: XConn>(list: &'static [&'static str]) -> impl StateHook<X> {
        move |_: &mut State<X>, _: &X| list.iter().try_for_each(|p| penrose::util::spawn(*p))
    }

    pub fn manage_place_at_tail(
        id: Xid,
        state: &mut State<RustConn>,
        _conn: &RustConn,
    ) -> penrose::Result<()> {
        if let Some(c) = state.client_set.remove_client(&id) {
            state.client_set.insert_at(penrose::pure::Position::Tail, c);
            state.client_set.focus_client(&c);
        }
        Ok(())
    }
}
