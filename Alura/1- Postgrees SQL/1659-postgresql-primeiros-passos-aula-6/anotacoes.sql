CREATE TABLE funcionarios(
    id SERIAL PRIMARY KEY,
    matricula VARCHAR(10),
    nome VARCHAR(255),
    sobrenome VARCHAR(255)
);

INSERT INTO funcionarios (matricula, nome, sobrenome) VALUES ('M001', 'Diogo', 'Mascarenhas');
INSERT INTO funcionarios (matricula, nome, sobrenome) VALUES ('M002', 'Vinícius', 'Dias');
INSERT INTO funcionarios (matricula, nome, sobrenome) VALUES ('M003', 'Nico', 'Steppat');
INSERT INTO funcionarios (matricula, nome, sobrenome) VALUES ('M004', 'João', 'Roberto');
INSERT INTO funcionarios (matricula, nome, sobrenome) VALUES ('M005', 'Diogo', 'Mascarenhas');
INSERT INTO funcionarios (matricula, nome, sobrenome) VALUES ('M006', 'Alberto', 'Martins');


select * from funcionarios

-- ordernar por nome
select * from funcionarios order by funcionarios.nome,sobrenome asc, 2 desc --ordena por nome, sobrenome, matricula, respectivamente (coluna) [varias formas de colocar]

-- limitando as consultas > para nao trazer muitos dados de uma tabela grande

select * from funcionarios
order by id
limit 5 --limita para buscar os 5 primeiros resultados apenas
offset 1 -- quantas linhas, a partir do começo, vou pular

--funcoes de agregacao
select count (id), sum (id), max(id) ,min(id), round(avg (id),2) from funcionarios

--agrupamento
select distinct nome from funcionarios -- distinct nao mostra nos resultados iguais
order by nome

-- se quisermos usar uma funcao de agregação, temos que usar group by
select nome, sobrenome, count(id) from funcionarios
group by nome, sobrenome
order by nome

--filtrando buscas agrupadas
select * from aluno
select * from aluno_curso
select * from curso

-- quais cursos nao tem aluno matriculado?
select curso.nome, count(aluno.id)
from curso left join aluno_curso on curso.id = aluno_curso.curso_id
left join aluno on aluno_curso.aluno_id = aluno.id
group by 1
having count(aluno.id) = 0 -- 'where' usado quando há func de agrupamento





