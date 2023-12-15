-- DROP SCHEMA public;

CREATE SCHEMA public AUTHORIZATION pg_database_owner;

COMMENT ON SCHEMA public IS 'standard public schema';
-- public.tag definition

-- Drop table

-- DROP TABLE public.tag;

CREATE TABLE public.tag (
	tag_id int8 NOT NULL, -- 标签ID
	tag_name varchar(256) NULL, -- 标签名称
	create_at timestamptz NOT NULL, -- 创建时间
	update_at timestamptz NULL, -- 更新时间
	is_valid bool NOT NULL DEFAULT true, -- 是否有效，假删除
	remark varchar(256) NULL DEFAULT ''::character varying, -- 备注
	CONSTRAINT tag_pk PRIMARY KEY (tag_id)
);
CREATE INDEX tag_tag_id_idx ON public.tag USING btree (tag_id DESC);
CREATE INDEX tag_tag_name_idx ON public.tag USING btree (tag_name);
COMMENT ON TABLE public.tag IS '标签';

-- Column comments

COMMENT ON COLUMN public.tag.tag_id IS '标签ID';
COMMENT ON COLUMN public.tag.tag_name IS '标签名称';
COMMENT ON COLUMN public.tag.create_at IS '创建时间';
COMMENT ON COLUMN public.tag.update_at IS '更新时间';
COMMENT ON COLUMN public.tag.is_valid IS '是否有效，假删除';
COMMENT ON COLUMN public.tag.remark IS '备注';

-- Permissions

ALTER TABLE public.tag OWNER TO postgres;
GRANT ALL ON TABLE public.tag TO postgres;


-- public.block definition

-- Drop table

-- DROP TABLE public.block;

CREATE TABLE public.block (
	block_id int8 NOT NULL, -- 块ID
	block_name varchar(24) NOT NULL, -- 块名称
	parent_id int8 NULL DEFAULT 0, -- 父ID
	create_at timestamptz NOT NULL, -- 创建时间
	update_at timestamptz NULL, -- 更新时间
	is_valid bool NOT NULL DEFAULT true, -- 是否有效，假删除
	remark varchar(256) NULL DEFAULT ''::character varying, -- 备注
	CONSTRAINT block_pk PRIMARY KEY (block_id)
);
CREATE INDEX block_block_name_idx ON public.block USING btree (block_name);
CREATE INDEX block_parent_id_idx ON public.block USING btree (parent_id);
COMMENT ON TABLE public.block IS '块';

-- Column comments

COMMENT ON COLUMN public.block.block_id IS '块ID';
COMMENT ON COLUMN public.block.block_name IS '块名称';
COMMENT ON COLUMN public.block.parent_id IS '父ID';
COMMENT ON COLUMN public.block.create_at IS '创建时间';
COMMENT ON COLUMN public.block.update_at IS '更新时间';
COMMENT ON COLUMN public.block.is_valid IS '是否有效，假删除';
COMMENT ON COLUMN public.block.remark IS '备注';

-- Permissions

ALTER TABLE public.block OWNER TO postgres;
GRANT ALL ON TABLE public.block TO postgres;


-- public.block_table definition

-- Drop table

-- DROP TABLE public.block_table;

CREATE TABLE public.block_table (
	block_table_id int8 NOT NULL, -- 块数据表格ID
	block_id int8 NOT NULL, -- 块ID
	create_at timestamptz NOT NULL, -- 创建时间
	update_at timestamptz NULL, -- 更新时间
	is_valid bool NOT NULL DEFAULT true, -- 是否有效，假删除
	remark varchar(256) NULL DEFAULT ''::character varying, -- 备注
	CONSTRAINT block_table_pk PRIMARY KEY (block_table_id)
);
CREATE INDEX block_table_block_id_idx ON public.block_table USING btree (block_id);
COMMENT ON TABLE public.block_table IS '块表格数据';

-- Column comments

COMMENT ON COLUMN public.block_table.block_table_id IS '块数据表格ID';
COMMENT ON COLUMN public.block_table.block_id IS '块ID';
COMMENT ON COLUMN public.block_table.create_at IS '创建时间';
COMMENT ON COLUMN public.block_table.update_at IS '更新时间';
COMMENT ON COLUMN public.block_table.is_valid IS '是否有效，假删除';
COMMENT ON COLUMN public.block_table.remark IS '备注';

-- Permissions

ALTER TABLE public.block_table OWNER TO postgres;
GRANT ALL ON TABLE public.block_table TO postgres;


-- public.block_table_column definition

-- Drop table

-- DROP TABLE public.block_table_column;

CREATE TABLE public.block_table_column (
	block_table_column_id int8 NOT NULL, -- 块表格列ID
	block_table_column_name varchar(256) NOT NULL, -- 块表格列名称
	create_at timestamptz NOT NULL, -- 创建时间
	update_at timestamptz NULL, -- 更新时间
	is_valid bool NOT NULL DEFAULT true, -- 是否有效，假删除
	remark varchar(256) NULL DEFAULT ''::character varying, -- 备注
	CONSTRAINT block_table_column_pk PRIMARY KEY (block_table_column_id)
);
CREATE INDEX block_table_column_block_table_column_name_idx ON public.block_table_column USING btree (block_table_column_name);
COMMENT ON TABLE public.block_table_column IS '块表格列';

-- Column comments

COMMENT ON COLUMN public.block_table_column.block_table_column_id IS '块表格列ID';
COMMENT ON COLUMN public.block_table_column.block_table_column_name IS '块表格列名称';
COMMENT ON COLUMN public.block_table_column.create_at IS '创建时间';
COMMENT ON COLUMN public.block_table_column.update_at IS '更新时间';
COMMENT ON COLUMN public.block_table_column.is_valid IS '是否有效，假删除';
COMMENT ON COLUMN public.block_table_column.remark IS '备注';

-- Permissions

ALTER TABLE public.block_table_column OWNER TO postgres;
GRANT ALL ON TABLE public.block_table_column TO postgres;


-- public.block_table_data definition

-- Drop table

-- DROP TABLE public.block_table_data;

CREATE TABLE public.block_table_data (
    block_table_data_id int8 NOT NULL, -- 块表格数据ID
	block_table_column_id int8 NOT NULL, -- 块表格列ID
	block_table_data_type int2 NOT NULL, -- 块表格数据类型
	block_table_data text NULL, -- 数据
	create_at timestamptz NOT NULL, -- 创建时间
	update_at timestamptz NULL, -- 更新时间
	is_valid bool NOT NULL DEFAULT true, -- 是否有效，假删除
	remark varchar(256) NULL DEFAULT ''::character varying, -- 备注
	CONSTRAINT block_table_data_pk PRIMARY KEY (block_table_data_id)
);
CREATE INDEX block_table_data_block_table_column_id_idx ON public.block_table_data USING btree (block_table_column_id);
COMMENT ON TABLE public.block_table_data IS '块表格数据';

-- Column comments

COMMENT ON COLUMN public.block_table_data.block_table_column_id IS '块表格列ID';
COMMENT ON COLUMN public.block_table_data.block_table_data_type IS '块表格数据类型';
COMMENT ON COLUMN public.block_table_data.block_table_data IS '数据';
COMMENT ON COLUMN public.block_table_data.create_at IS '创建时间';
COMMENT ON COLUMN public.block_table_data.update_at IS '更新时间';
COMMENT ON COLUMN public.block_table_data.is_valid IS '是否有效，假删除';
COMMENT ON COLUMN public.block_table_data.remark IS '备注';
COMMENT ON COLUMN public.block_table_data.block_table_data_id IS '块表格数据ID';

-- Permissions

ALTER TABLE public.block_table_data OWNER TO postgres;
GRANT ALL ON TABLE public.block_table_data TO postgres;




-- Permissions

GRANT ALL ON SCHEMA public TO pg_database_owner;
GRANT USAGE ON SCHEMA public TO public;