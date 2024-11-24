drop table if exists course;


create table `course` (
  `id` serial primary key,
  `teacher_id` int Not null,
  `name` varchar(140) not null,
  `time` timestamp Default now()
);


insert into course (id, teacher_id, name, time) values (1, 1, 'First Course', '2020-01-01 00:00:00');
insert into course (id, teacher_id, name, time) values (2, 1, 'Second Course', '2020-01-01 00:45:00');
insert into course (id, teacher_id, name, time) values (3, 1, 'Third Course', '2020-01-01 00:45:00');
