select * from aluno
select * from curso
select * from aluno_curso

-- restrições de chave estrangeira
-- olha na criação da tabela aluno_curso

create table aluno_curso (
	aluno_id integer, 
	curso_id integer, 
	primary key (aluno_id, curso_id),
	foreign key (aluno_id) references aluno (id),
	foreign key (curso_id) references curso (id)
)

-- adicionar restrição na chave estrangeira

create table aluno_curso (
	aluno_id integer, 
	curso_id integer, 
	primary key (aluno_id, curso_id),
	foreign key (aluno_id) references aluno (id) on delete cascade on update cascade, --comportamento que deleta e atualiza em cascada, ou seja, deletar ou atualizar um aluno tb deleta ou atualiza de todos os cursos que ele está associado
	foreign key (curso_id) references curso (id)
)

drop table aluno_curso

insert into aluno_curso (aluno_id, curso_id) values (1,1)
insert into aluno_curso (aluno_id, curso_id) values (2,1)
insert into aluno_curso (aluno_id, curso_id) values (3,1)
insert into aluno_curso (aluno_id, curso_id) values (1,3)

select * from aluno_curso

select aluno.nome, curso.nome
from aluno join aluno_curso on aluno_curso.aluno_id = aluno.id
join curso on curso.id = aluno_curso.curso_id
-- mostra que diogo esta matriculado em javascript e html

delete from aluno where id = 1 -- posso fazer isso pois, com o cascade, deletei todas as relações desse elemento

-- UPDATE CASCADE
update aluno set id = 10 where id = 2; -- nao posso fazer isso aqui pois esse i já esta sendo usado em outra coisa


