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


alter table Course 
add column decription varchar(2000),
add column format varchar(140),
add column structure varchar(100),
add column duration varchar(40),
add column price Integer,
add column language varchar(60),
add column level varchar(40);


create table teacher (
  id serial not null primary key,
  name varchar(100),
  picture_url varchar(255),
  profile varchar(1000)
);


create table teacher (
  id integer default nextval('teacher_id_seq'::regclass), -- part of primary key
  name varchar(100),
  picture_url varchar(255),
  profile varchar(1000)
);
