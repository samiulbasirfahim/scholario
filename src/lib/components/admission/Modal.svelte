<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import InputContainer from '../global/InputContainer.svelte';
	import { onMount } from 'svelte';
	import type { Guardian } from '$lib/type/guardian';
	import Icon from '@iconify/svelte';

	let { selected_guardians = $bindable([]) } = $props();
	let numberConflictGuardian = $state(false);
	let search_term = $state('');
	let guardianForm = $state({
		name: 'Samiul Basir Fahim',
		phone: '1728071074',
		address: 'Mymensingh',
		photo: '',
		relation: ''
	});

	let selected_guardian: Guardian | undefined = $state();
	let guardians: Guardian[] = $state([]);
	let guardians_d = $derived.by(() =>
		guardians.filter((g1) => !selected_guardians.some((g2) => g1.phone === g2.phone))
	);

	const add_guardian = () => {
		if (selected_guardian) {
			if (selected_guardians.findIndex((g) => g.id === selected_guardian?.id) < 0) {
				selected_guardians.push({ ...selected_guardian, relation: guardianForm.relation });
			}
			(selected_guardian = undefined), (guardianForm.relation = '');
		}
	};

	const create_guardian = async () => {
		let g = await invoke('guardian_save', {
			...guardianForm,
			phone: `+880${guardianForm.phone}`
		});

		console.log(g);

		document.getElementById('create-guardian-button')?.click();
	};

	const guardian_search = () => {
		invoke('guardian_search', { term: search_term }).then((d: Guardian[]) => {
			guardians = d;
		});
	};

	$effect(() => {
		console.log($state.snapshot(guardians));
	});
</script>

<input type="checkbox" id="create-guardian" class="modal-toggle" />
<div class="modal px-4">
	<form
		class="modal-box p-4"
		onsubmit={(e) => {
			e.preventDefault();
			create_guardian();
		}}
	>
		<h3 class="text-lg font-bold">Create Guardian</h3>
		<div class="py-4">
			<InputContainer name="name" label="Full name: ">
				<input
					type="text"
					name="name"
					required
					minlength="5"
					class="input w-full"
					bind:value={guardianForm.name}
				/>
			</InputContainer>
			<InputContainer name="phone" label="Phone: ">
				<label class="input">
					+880
					<input
						type="tel"
						class="validator grow tabular-nums"
						required
						pattern="[0-9]*"
						minlength="10"
						maxlength="10"
						title="Must be 10 digits"
						bind:value={guardianForm.phone}
					/>
					<p class="validator-hint">Must be 10 digits</p>
				</label>
			</InputContainer>

			<InputContainer name="address" label="Address: ">
				<input
					required
					minlength="10"
					type="text"
					name="address"
					class="input w-full"
					bind:value={guardianForm.address}
				/>
			</InputContainer>

			<InputContainer name="relation" label="Phone: ">
				<input
					type="text"
					class="input w-full"
					id="relation"
					placeholder="Relation"
					list="relations"
					required
					bind:value={guardianForm.relation}
				/>
				<datalist id="relations">
					{#each ['Father', 'Mother', 'Uncle', 'Grandpa'] as rel (rel)}
						<option value={rel}> </option>
					{/each}
					>
				</datalist></InputContainer
			>

			<InputContainer name="photo" label="Photo: ">
				<input
					type="file"
					class="file-input file-input-primary w-full"
					name="photo"
					accept="image/*"
					onchange={(e) => {
						let files = (e.target as HTMLInputElement).files;
						if (!files) return;

						let file = files[0];

						const reader = new FileReader();

						reader.onload = (ev) => {
							const base64String = ev.target.result.split(',')[1];
							guardianForm.photo = 'data:image/png;base64,' + base64String;
						};

						reader.readAsDataURL(file);
					}}
				/>
			</InputContainer>
		</div>

		{#if numberConflictGuardian}
			<div class="alert alert-error">
				<span>You cant have two guardian with same phone number.</span>
			</div>
		{/if}
		<div class="modal-action">
			<div class="flex items-center justify-center gap-4">
				<label for="create-guardian" class="btn btn-primary" id="create-guardian-button"
					>Close</label
				>
				<input class="btn btn-primary" type="submit" value="Save" />
			</div>
		</div>
	</form>
</div>

<input type="checkbox" id="find-guardian" class="modal-toggle" />
<div class="modal px-4">
	<div class="modal-box bg-base-200 w-full p-4 sm:col-span-2 lg:col-span-1 2xl:col-span-2">
		<h3 class="text-lg font-bold">Find Guardian</h3>

		<InputContainer name="phone" label="Phone: " class="bg-base-200 py-4">
			<input type="text" bind:value={search_term} class="input w-full" oninput={guardian_search} />
		</InputContainer>
		<div
			class="list card rounded-box grid max-h-[60vh] w-full grow grid-cols-1 gap-4 overflow-y-auto p-4"
		>
			{#each guardians_d as guardian (guardian.id)}
				<li class="list-row bg-base-100 flex items-center justify-between">
					<div class="flex items-center justify-start">
						<div class="avatar">
							<div class="rounded-box w-16">
								<img src={guardian.photo} alt="{guardian.name}'s photo" />
							</div>
						</div>
						<div class="ps-2">
							<div class="text-base font-semibold">{guardian.name}</div>
							<div class="text-sm font-medium uppercase">{guardian.phone}</div>
							<div class="text-xs font-light">{guardian.address}</div>
						</div>
					</div>

					<button
						class="btn btn-primary"
						onclick={() => {
							selected_guardian = guardian;
							document.getElementById('rel-guardian-button')?.click();
						}}
					>
						<Icon icon="mdi:add-bold" font-size="26" />
					</button>
				</li>
			{/each}
		</div>
		<div class="modal-action">
			<label for="find-guardian" class="btn btn-primary">Close</label>
		</div>
	</div>
</div>

<input type="checkbox" id="rel-guardian" class="modal-toggle" />
<div class="modal">
	<form
		class="modal-box bg-base-200 w-full p-4 sm:col-span-2 lg:col-span-1 2xl:col-span-2"
		onsubmit={(e) => {
			e.preventDefault();
			add_guardian();
			document.getElementById('rel-guardian-button')?.click();
		}}
	>
		<h3 class="text-lg font-bold">Relation</h3>

		<InputContainer name="relation" label="Phone: " class="bg-base-200 py-4">
			<input
				type="text"
				class="input w-full"
				id="relation"
				placeholder="Relation"
				list="relations"
				required
				bind:value={guardianForm.relation}
			/>
			<datalist id="relations">
				{#each ['Father', 'Mother', 'Uncle', 'Grandpa'] as rel (rel)}
					<option value={rel}> </option>
				{/each}
				>
			</datalist></InputContainer
		>
		<div class="modal-action">
			<label for="rel-guardian" class="btn btn-primary hidden" id="rel-guardian-button"></label>
			<input type="submit" value="Save" class="btn btn-primary" />
		</div>
	</form>
</div>
