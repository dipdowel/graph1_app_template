use crate::bouncy::{BOUNCY_USER_DATA, BouncyUserData};

// Shape the user-defined data, accessible via `ctx.user_data`
// `ctx.user_data` can be used to store arbitrary data that needs to live as long as the context itself.
// E.g. the state of the animation, the position of the object, or anything else that needs
// to be persisted between frames.
pub struct DemoUserData {
    pub bouncy: BouncyUserData,
}

// Populate the user data with values defined in the corresponding demo modules
impl<'a> Default for DemoUserData {
    fn default() -> Self {
        DemoUserData {
            bouncy: BOUNCY_USER_DATA,
        }
    }
}
