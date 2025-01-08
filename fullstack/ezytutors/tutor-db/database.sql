
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=postgres  postgres
docker exec -it f82accd9a924 psql -U <username>

create database ezytutors;
create user truuser with password 'mypassword';
grant all privileges on database ezytutors to truuser;
GRANT CREATE ON SCHEMA public TO truuser;


docker exec -it f82accd9a924 psql -U truuser -d ezytutors --password

/* Drop table if it already exists*/
drop table if exists ezy_course_c4;
/* Create table. */
/* Note: Don't put a comma after last field */
create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* Load seed data for testing */
insert into ezy_course_c4
    (course_id,tutor_id, course_name,posted_time)
values(1, 1, 'First course', '2020-12-17 05:40:00');
insert into ezy_course_c4
    (course_id, tutor_id, course_name,posted_time)
values(2, 1, 'Second course', '2020-12-18 05:45:00');