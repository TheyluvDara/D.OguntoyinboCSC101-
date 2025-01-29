                        Table "public.staff"
   Column   |         Type          | Collation | Nullable | Default 
------------+-----------------------+-----------+----------+---------
 staff_id   | integer               |           | not null | 
 staff_name | text                  |           | not null | 
 dno        | integer               |           | not null | 
 staff_sal  | real                  |           | not null | 
 age        | integer               |           | not null | 
 mobile     | character varying(15) |           | not null | 
Indexes:
    "employees_pkey" PRIMARY KEY, btree (staff_id)

