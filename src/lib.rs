pub mod hooks {
    use penrose::{core::State, x11rb::RustConn, Xid};

    pub fn startup_progs(_state: &mut State<RustConn>, _conn: &RustConn) -> penrose::Result<()> {
        // penrose::util::spawn("picom")?;
        penrose::util::spawn("/home/volt/.fehbg")
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
