<script lang="ts">
	import InputContainer from '$lib/components/global/InputContainer.svelte';

	let classes = ['primary', 'secondery', 'intermediate', 'higher'];

	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Guardian } from '$lib/type/guardian';
	import { onMount } from 'svelte';

	let numberConflictGuardian = false;
	let input_phone = $state('');

	let guardianForm = $state({
		name: 'Samiul Basir Fahim',
		phone: '1728071074',
		address: 'Mymensingh',
		photo: ''
	});

	let relation: string = $state('');

	let selected_guardians: Guardian[] = $state([]);
	let selected_guardian: Guardian | undefined = $state();

	const add_guardian = () => {
		if (selected_guardian) {
			if (selected_guardians.findIndex((g) => g.id === selected_guardian?.id) < 0) {
				selected_guardians.push({ ...selected_guardian, relation });
			}
			(selected_guardian = undefined), (relation = '');
		}
	};
	const remove_guardian = (id: number | undefined) => {
		selected_guardians = selected_guardians.filter((g) => g.id !== id);
	};

	let guardians: Guardian[] = $state([]);
	const guardian_search = () => {
		invoke('guardian_search', { phone: input_phone }).then((d: Guardian[]) => {
			guardians = d;
		});
	};
	onMount(guardian_search);
</script>

<div class="flex w-full flex-col p-8 lg:flex-row">
	<div class="flex items-start justify-start">
		<div
			class="card bg-base-300 rounded-box grid grow grid-cols-1 place-items-center gap-6 p-6 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" />
			</InputContainer>

			<InputContainer name="phone" label="Phone: ">
				<label class="input">
					+880
					<input
						type="tel"
						class="validator w-full grow tabular-nums"
						required
						pattern="[0-9]*"
						minlength="10"
						maxlength="10"
						title="Must be 10 digits"
					/>
					<p class="validator-hint">Must be 10 digits</p>
				</label>
			</InputContainer>

			<InputContainer
				name="address"
				label="Address: "
				class="w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2"
			>
				<input type="text" name="address" class="input w-full" />
			</InputContainer>
			<ul class="bg-base-200 w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2">
				<li class="p-4 pb-2 text-sm tracking-wide">Guardians</li>
				<div
					class="list card rounded-box grid max-h-54 w-full grow grid-cols-1 gap-4 overflow-y-auto px-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
				>
					{#each selected_guardians as guardian (guardian.id)}
						<li class="list-row bg-base-100 flex items-center justify-between">
							<div class="flex items-center justify-start">
								<div class="avatar">
									<div class="rounded-box w-16">
										<img src={guardian.photo} alt="{guardian.name}'s photo" />
									</div>
								</div>
								<div class="ps-2">
									<div class="text-xs font-light">{guardian.relation}</div>
									<div class="text-base font-semibold">{guardian.name}</div>
									<div class="text-sm font-medium uppercase">{guardian.phone}</div>
									<div class="text-xs font-light">{guardian.address}</div>
								</div>
							</div>
							<button class="btn btn-square btn-ghost" onclick={() => remove_guardian(guardian.id)}>
								<Icon icon="fa:remove" font-size="16" />
							</button>
						</li>
					{/each}
				</div>
				<div class="flex flex-wrap justify-between gap-4 p-4">
					<label for="create-guardian" class="btn btn-primary">Create Guardian</label>
					<label for="find-guardian" class="btn btn-primary">Find Guardian</label>
				</div>
			</ul>
		</div>
	</div>
	<div class="divider lg:divider-horizontal"></div>
	<div class="flex items-start justify-start">
		<div
			class="card bg-base-300 rounded-box grid grow grid-cols-1 place-items-center gap-6 p-6 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="class" label="Class: ">
				<select class="select" id="class">
					<option disabled selected>Class</option>
					{#each classes as cls (cls)}
						<option value={cls}>{cls}</option>
					{/each}
				</select>
			</InputContainer>

			<InputContainer name="section" label="Section: ">
				<select class="select" id="section">
					<option disabled selected>Section</option>
					{#each classes as cls (cls)}
						<option value={cls}>{cls}</option>
					{/each}
				</select>
			</InputContainer>

			<InputContainer name="blood-group" label="Blood group: ">
				<select class="select" id="blood-group">
					<option disabled selected>Blood group</option>
					<option value="A+">A+</option>
					<option value="A-">A-</option>
					<option value="B+">B+</option>
					<option value="B-">B-</option>
					<option value="AB+">AB+</option>
					<option value="AB-">AB-</option>
					<option value="O+">O+</option>
					<option value="O-">O-</option>
					<option value="unknown">Unknown</option>
				</select>
			</InputContainer>

			<InputContainer name="dob" label="Date of birth: ">
				<input type="date" class="input" name="dob" />
			</InputContainer>

			<InputContainer name="photo" label="Photo: ">
				<input type="file" class="file-input file-input-primary w-full" name="photo" />
			</InputContainer>

			<InputContainer name="resident" label="Resident">
				<input type="checkbox" checked class="checkbox" />
			</InputContainer>
		</div>
	</div>
</div>

<input type="checkbox" id="create-guardian" class="modal-toggle" />
<div class="modal px-4">
	<form class="modal-box p-4">
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

			<InputContainer name="photo" label="Photo: ">
				<input
					type="file"
					class="file-input file-input-primary w-full"
					name="photo"
					accept="image/*"
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
			<input type="phone" bind:value={input_phone} class="input w-full" oninput={guardian_search} />
		</InputContainer>
		<div
			class="list card rounded-box grid max-h-[60vh] w-full grow grid-cols-1 gap-4 overflow-y-auto p-4"
		>
			{#each guardians as guardian (guardian.id)}
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
				bind:value={relation}
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
