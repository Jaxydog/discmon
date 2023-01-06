use crate::prelude::*;

macro_rules! traits {
    ($try:ident { $try_fn:ident }, $to:ident { $to_fn:ident }, $async:ident { $async_fn:ident } => $output:ty) => {
        pub trait $try {
            type Args;

            fn $try_fn(&self, _: Self::Args) -> Result<$output>;
        }
        pub trait $to {
            type Args;

            fn $to_fn(&self, _: Self::Args) -> $output;
        }
        #[async_trait]
        pub trait $async {
            type Args: Send + Sync;

            async fn $async_fn(&self, http: &impl CacheHttp, _: Self::Args) -> Result<$output>;
        }

        impl<T: $to> $try for T {
            type Args = T::Args;

            fn $try_fn(&self, args: Self::Args) -> Result<$output> {
                Ok(self.$to_fn(args))
            }
        }
    };
    (disableable $try:ident { $try_fn:ident }, $to:ident { $to_fn:ident }, $async:ident { $async_fn:ident } => $output:ty) => {
        pub trait $try {
            type Args;

            fn $try_fn(&self, disabled: bool, _: Self::Args) -> Result<$output>;
        }
        pub trait $to {
            type Args;

            fn $to_fn(&self, disabled: bool, _: Self::Args) -> $output;
        }
        #[async_trait]
        pub trait $async {
            type Args: Send + Sync;

            async fn $async_fn(
                &self,
                disabled: bool,
                http: &impl CacheHttp,
                _: Self::Args,
            ) -> Result<$output>;
        }

        impl<T: $to> $try for T {
            type Args = T::Args;

            fn $try_fn(&self, disabled: bool, args: Self::Args) -> Result<$output> {
                Ok(self.$to_fn(disabled, args))
            }
        }
    };
}

traits!(disableable TryToButton { try_to_button }, ToButton { to_button }, ToButtonAsync { to_button } => CreateButton);
traits!(disableable TryToButtons { try_to_buttons }, ToButtons { to_buttons }, ToButtonsAsync { to_buttons } => Vec<CreateButton>);
traits!(TryToEmbed { try_to_embed }, ToEmbed { to_embed }, ToEmbedAsync { to_embed } => CreateEmbed);
traits!(TryToInputText { try_to_input_text }, ToInputText { to_input_text }, ToInputTextAsync { to_input_text } => CreateInputText);
traits!(TryToMessage { try_to_message }, ToMessage { to_message }, ToMessageAsync { to_message } => CreateMessage);
traits!(TryToModal { try_to_modal }, ToModal { to_modal }, ToModalAsync { to_modal } => CreateModal);
