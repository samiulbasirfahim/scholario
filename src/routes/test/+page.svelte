<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="space-y-8 p-6">
	<!-- Header Cards -->
	<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-4">
		<div class="stats from-primary to-secondary text-primary-content bg-gradient-to-br shadow">
			<div class="stat">
				<div class="stat-title opacity-80">Total Students</div>
				<div class="stat-value text-white">1,245</div>
				<div class="stat-desc opacity-70">↗︎ 5.2% increase</div>
			</div>
		</div>
		<div class="stats from-secondary to-accent text-secondary-content bg-gradient-to-br shadow">
			<div class="stat">
				<div class="stat-title opacity-80">Total Staff</div>
				<div class="stat-value text-white">76</div>
				<div class="stat-desc opacity-70">+3 this month</div>
			</div>
		</div>
		<div class="stats from-accent to-primary text-accent-content bg-gradient-to-br shadow">
			<div class="stat">
				<div class="stat-title opacity-80">Attendance Today</div>
				<div class="stat-value text-white">92%</div>
				<div class="stat-desc opacity-70">Excellent</div>
			</div>
		</div>
		<div class="stats from-neutral to-base-300 text-neutral-content bg-gradient-to-br shadow">
			<div class="stat">
				<div class="stat-title opacity-80">Broadcasts Sent</div>
				<div class="stat-value text-white">14</div>
				<div class="stat-desc opacity-70">↓ 2 this week</div>
			</div>
		</div>
	</div>

	<!-- Main Graph & Feed -->
	<div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
		<div class="bg-base-100 border-base-300 col-span-2 rounded-xl border p-6 shadow-md">
			<h2 class="text-primary mb-4 text-xl font-bold">Attendance This Week</h2>
			<div class="overflow-x-auto">
				<table class="table w-full">
					<thead>
						<tr>
							<th>Day</th>
							<th>Present</th>
							<th>Absent</th>
							<th>Late</th>
						</tr>
					</thead>
					<tbody>
						<tr><td>Monday</td><td>1120</td><td>75</td><td>50</td></tr>
						<tr><td>Tuesday</td><td>1134</td><td>63</td><td>48</td></tr>
						<tr><td>Wednesday</td><td>1150</td><td>52</td><td>43</td></tr>
						<tr><td>Thursday</td><td>1165</td><td>40</td><td>35</td></tr>
						<tr><td>Friday</td><td>1180</td><td>25</td><td>20</td></tr>
					</tbody>
				</table>
			</div>
		</div>

		<!-- Notification Feed -->
		<div class="bg-base-100 border-base-300 rounded-xl border p-6 shadow-md">
			<h2 class="text-primary mb-4 text-xl font-bold">Notifications</h2>
			<ul class="space-y-4">
				<li class="flex items-start gap-3">
					<Icon icon="mdi:bell-outline" class="text-warning text-xl" />
					<div>
						<p class="text-base-content font-medium">Fire drill scheduled on Jun 20</p>
						<p class="text-sm opacity-70">Make sure all classes are aware.</p>
					</div>
				</li>
				<li class="flex items-start gap-3">
					<Icon icon="mdi:calendar-check-outline" class="text-success text-xl" />
					<div>
						<p class="text-base-content font-medium">Parent-Teacher meeting next Monday</p>
						<p class="text-sm opacity-70">Reminders will be sent tomorrow.</p>
					</div>
				</li>
				<li class="flex items-start gap-3">
					<Icon icon="mdi:email-outline" class="text-info text-xl" />
					<div>
						<p class="text-base-content font-medium">New broadcast templates available</p>
						<p class="text-sm opacity-70">Visit the Broadcasts page.</p>
					</div>
				</li>
			</ul>
		</div>
	</div>

	<!-- Results and Upcoming Events -->
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
		<!-- Exam Results -->
		<div class="bg-base-100 border-base-300 rounded-xl border p-6 shadow-md">
			<h2 class="text-primary mb-4 text-xl font-bold">Latest Exam Results</h2>
			<ul class="space-y-2">
				<li>
					<span class="font-semibold">Class 10</span> - Math: <span class="text-success">87%</span> avg
					score
				</li>
				<li>
					<span class="font-semibold">Class 9</span> - English:
					<span class="text-warning">72%</span>
				</li>
				<li>
					<span class="font-semibold">Class 8</span> - Science: <span class="text-error">65%</span>
				</li>
				<li>
					<span class="font-semibold">Class 7</span> - Bangla: <span class="text-success">91%</span>
				</li>
			</ul>
		</div>

		<!-- Upcoming Events -->
		<div class="bg-base-100 border-base-300 rounded-xl border p-6 shadow-md">
			<h2 class="text-primary mb-4 text-xl font-bold">Upcoming Events</h2>
			<ul class="space-y-2">
				<li>📅 <span class="font-semibold">Science Fair</span> — July 10</li>
				<li>📅 <span class="font-semibold">Annual Sports Day</span> — August 5</li>
				<li>📅 <span class="font-semibold">Final Exams Begin</span> — September 12</li>
			</ul>
		</div>
	</div>
</div>

<!-- INSTALLATION: DaisyUI -->
<!-- If you haven’t installed DaisyUI yet: -->
<!-- Install Tailwind CSS first, then run: -->
<!-- npm install -D daisyui -->
<!-- And in tailwind.config.{js,ts}, add: -->
<!-- plugins: [require('daisyui')], -->
<!-- That’s all. No extra imports needed. -->
