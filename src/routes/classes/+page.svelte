<script lang="ts">
	import InputContainer from '$lib/components/global/InputContainer.svelte';
	import Icon from '@iconify/svelte';

	let subjects = $state([
		{
			id: 1,
			name: 'Mathematics'
		},

		{
			id: 10,
			name: 'Mathematics'
		},
		{
			id: 11,
			name: 'Mathematics'
		},
		{
			id: 111,
			name: 'Mathematics'
		},
		{
			id: 2,
			name: 'English'
		},
		{
			id: 3,
			name: 'Science'
		},
		{
			id: 4,
			name: 'Art'
		},
		{
			id: 5,
			name: 'Physical Education'
		},
		{
			id: 6,
			name: 'History'
		}
	]);

	let teachers = $state([
		'Emily Johnson',
		'David Miller',
		'Priya Patel',
		'James Brown',
		'Sarah Davis'
	]);

	let classes = $state([
		{
			id: 1,
			name: 'Grade 3',
			sections: 2,
			monthly_fee: 15.0,
			subjects: 5,
			class_teacher: 'Ms. Emily',
			students: 40
		},
		{
			id: 2,
			name: 'Grade 7',
			sections: 3,
			monthly_fee: 25.5,
			subjects: 7,
			class_teacher: 'Mr. David',
			students: 60
		},
		{
			id: 3,
			name: 'Grade 10',
			sections: 4,
			monthly_fee: 30.75,
			subjects: 8,
			class_teacher: 'Mrs. Patel',
			students: 80
		},
		{
			id: 1,
			name: 'Grade 3',
			sections: 2,
			monthly_fee: 15.0,
			subjects: 5,
			class_teacher: 'Ms. Emily',
			students: 40
		},
		{
			id: 2,
			name: 'Grade 7',
			sections: 3,
			monthly_fee: 25.5,
			subjects: 7,
			class_teacher: 'Mr. David',
			students: 60
		},
		{
			id: 3,
			name: 'Grade 10',
			sections: 4,
			monthly_fee: 30.75,
			subjects: 8,
			class_teacher: 'Mrs. Patel',
			students: 80
		},
		{
			id: 1,
			name: 'Grade 3',
			sections: 2,
			monthly_fee: 15.0,
			subjects: 5,
			class_teacher: 'Ms. Emily',
			students: 40
		},
		{
			id: 2,
			name: 'Grade 7',
			sections: 3,
			monthly_fee: 25.5,
			subjects: 7,
			class_teacher: 'Mr. David',
			students: 60
		},
		{
			id: 3,
			name: 'Grade 10',
			sections: 4,
			monthly_fee: 30.75,
			subjects: 8,
			class_teacher: 'Mrs. Patel',
			students: 80
		},
		{
			id: 1,
			name: 'Grade 3',
			sections: 2,
			monthly_fee: 15.0,
			subjects: 5,
			class_teacher: 'Ms. Emily',
			students: 40
		},
		{
			id: 2,
			name: 'Grade 7',
			sections: 3,
			monthly_fee: 25.5,
			subjects: 7,
			class_teacher: 'Mr. David',
			students: 60
		},
		{
			id: 3,
			name: 'Grade 10',
			sections: 4,
			monthly_fee: 30.75,
			subjects: 8,
			class_teacher: 'Mrs. Patel',
			students: 80
		},
		{
			id: 1,
			name: 'Grade 3',
			sections: 2,
			monthly_fee: 15.0,
			subjects: 5,
			class_teacher: 'Ms. Emily',
			students: 40
		},
		{
			id: 2,
			name: 'Grade 7',
			sections: 3,
			monthly_fee: 25.5,
			subjects: 7,
			class_teacher: 'Mr. David',
			students: 60
		},
		{
			id: 3,
			name: 'Grade 10',
			sections: 4,
			monthly_fee: 30.75,
			subjects: 8,
			class_teacher: 'Mrs. Patel',
			students: 80
		}
	]);
</script>

<div class="flex w-full flex-col p-8 lg:flex-row">
	<div class="flex w-full items-start justify-start">
		<ul class="list bg-base-300 rounded-box max-h-[90vh] w-full gap-4 overflow-y-auto p-4">
			{#each classes as cls, i (i)}
				<li class="card bg-base-100 flex w-full flex-row items-start justify-between p-4">
					<div class="w-full">
						<h2 class="card-title text-xl uppercase">{cls.name}</h2>
						<p class="text-sm opacity-60">Teacher: {cls.class_teacher}</p>
						<div class="divider my-2"></div>
						<div class="grid grid-cols-2 text-sm opacity-60">
							<p>Total Students: {cls.students}</p>
							<p>Total Sections: {cls.sections}</p>
							<p>Monthly Fee: ${cls.monthly_fee.toFixed(2)}</p>
							<p>Subjects: {cls.subjects}</p>
						</div>
					</div>

					<div class="flex gap-2">
						<button class="btn btn-square btn-ghost opacity-40 hover:opacity-100">
							<Icon icon="material-symbols:delete-rounded" font-size="26" />
						</button>
						<button class="btn btn-square btn-ghost opacity-40 hover:opacity-100">
							<Icon icon="majesticons:open" font-size="26" />
						</button>
					</div>
				</li>
			{/each}
		</ul>
	</div>
	<div class="divider lg:divider-horizontal"></div>
	<div class="flex max-h-60 w-full flex-col items-start justify-start gap-4 overflow-y-visible">
		<form
			class="card bg-base-300 rounded-box grid w-full grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="subject" label="Subject: ">
				<input type="text" class="input" name="subject" />
			</InputContainer>

			<InputContainer name="+" label="+">
				<input type="submit" class="btn btn-primary w-full" value="Create subject" />
			</InputContainer>
		</form>
		<form
			class="card bg-base-300 rounded-box grid w-full grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="class" label="Class name: ">
				<input type="text" name="class" class="input w-full" />
			</InputContainer>

			<InputContainer name="monthly_fee" label="Monthly fee: ">
				<input type="number" name="monthly_fee" class="input w-full" />
			</InputContainer>

			<InputContainer name="teacher" label="Class teacher: ">
				<select name="teacher" class="select">
					<option value="">Teacher</option>
					{#each teachers as teacher}
						<option value={teacher}>{teacher}</option>
					{/each}
				</select>
			</InputContainer>

			<ul class="bg-base-200 w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2">
				<li class="p-4 pb-2 text-sm tracking-wide">Subjects</li>
				<div
					class="list card rounded-box grid max-h-54 w-full grow grid-cols-1 gap-4 overflow-y-auto px-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
				>
					{#each subjects as sub (sub.id)}
						<div class="list-row bg-base-100 flex items-center justify-between">
							<div>
								<input type="checkbox" class="checkbox" />
								{sub.name}
							</div>
							<div>
								<input type="checkbox" class="checkbox checkbox-xs" />
								<span class="text-xs opacity-60">Optional</span>
							</div>
						</div>
					{/each}
				</div>
			</ul>
		</form>
	</div>
</div>
