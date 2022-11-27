<script>
	import Code from '$lib/components/Code.svelte';
	import Details from '$lib/components/Details.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Info from '$lib/components/Info.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
</script>

<Note>
	<Title date="27 November 2022" subtitle="There are many like it, but this one is mine"
		>About this website</Title>
	<P>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		viverra. Quisque vel elit ex. Suspendisse finibus, metus sed sollicitudin ullamcorper, leo nunc
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		ipsum primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
	<Code
		value={`use crate::{error::AppError, shares::get_share_expiration, shares::KeyJson};

pub async fn access_share_handler(
    Path(token): Path<String>,
    db: Extension<PgPool>,
) -> Result<Response, AppError> {
    let expires_at = get_share_expiration(&token, &db).await?;

    let now = Utc::now();

    if let Some(expires) = expires_at {
        if expires < now {
            return Err(AppError::Unauthorized);
        }
    }

    let note = access_share(&token, &db).await?;

    Ok(Json(&note).into_response())
}
`} />
	<P>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		sem. Duis et tristique ex, et vehicula nisi. Morbi vitae auctor risus. Proin sit amet nunc ipsum
		primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
	<Hint>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		sem. Duis et tristique ex, et vehicula nisi. Morbi vitae auctor risus. Proin sit amet nunc
		ornare, dapibus tellus ac, pulvinar massa. Quisque vitae risus laoreet, tristique diam ac,
		mollis mauris. Ut id mollis libero. Sed rhoncus justo a tortor scelerisque, sed molestie metus
		cursus. Interdum et malesuada fames ac ante ipsum primis in faucibus.
	</Hint>
	<P>
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		ipsum primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
	<P>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		sem. Duis et tristique ex, et vehicula nisi. Morbi vitae auctor risus. Proin sit amet nunc ipsum
		primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
	<P>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		sem. Duis et tristique ex, et vehicula nisi. Morbi vitae auctor risus. Proin sit amet nunc ipsum
		primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
	<Details title="Read more about this">
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		sem. Duis et tristique ex, et vehicula nisi. Morbi vitae auctor risus. Proin sit amet nunc
		ornare, dapibus tellus ac, pulvinar massa. Quisque vitae risus laoreet, tristique diam ac,
		mollis mauris. Ut id mollis libero. Sed rhoncus justo a tortor scelerisque, sed molestie metus
		cursus. Interdum et malesuada fames ac ante ipsum primis in faucibus.
	</Details>
	<P>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		viverra. Quisque vel elit ex. Suspendisse finibus, metus sed sollicitudin ullamcorper, leo nunc
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		ipsum primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
	<Info>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		sem. Duis et tristique ex, et vehicula nisi. Morbi vitae auctor risus. Proin sit amet nunc
		ornare, dapibus tellus ac, pulvinar massa. Quisque vitae risus laoreet, tristique diam ac,
		mollis mauris. Ut id mollis libero. Sed rhoncus justo a tortor scelerisque, sed molestie metus
		cursus. Interdum et malesuada fames ac ante ipsum primis in faucibus.
	</Info>
	<P>
		This is a hint Lorem ipsum dolor sit amet, consectetur adipiscing elit. In luctus ac massa id
		viverra. Quisque vel elit ex. Suspendisse finibus, metus sed sollicitudin ullamcorper, leo nunc
		accumsan sapien, vitae condimentum ex lectus vitae nunc. Interdum et malesuada fames ac ante
		ipsum primis in faucibus. Donec et laoreet dolor. Morbi porttitor nec ipsum a vulputate.
	</P>
</Note>
