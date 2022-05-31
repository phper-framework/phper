#ifndef PHPER_PHP_WRAPPER_H
#define PHPER_PHP_WRAPPER_H

#include <stdbool.h>
#include <php.h>
#include <php_ini.h>
#include <ext/standard/info.h>
#include <zend_exceptions.h>
#include <main/SAPI.h>

typedef ZEND_INI_MH(phper_zend_ini_mh);

zend_string *zend_new_interned_string_(zend_string *str);
zend_class_entry phper_init_class_entry_ex(const char *class_name, size_t class_name_len, const zend_function_entry *functions);
zend_uchar phper_zval_get_type(const zval* pz);

void phper_zval_string(zval *return_value, const char *s);
void phper_zval_arr(zval *return_value, zend_array *arr);
void phper_zval_new_arr(zval *return_value);
void phper_zval_stringl(zval *return_value, const char *s, size_t len);

char *phper_z_strval_p(const zval *v);
zval *phper_get_this(zend_execute_data *execute_data);
void phper_zval_zval(zval *return_value, zval *zv, int copy, int dtor);
void phper_zval_dup(zval *return_value, zval *zv);
void phper_zval_copy(zval *return_value, zval *zv);
void phper_zval_copy_value(zval *return_value, zval *zv);

zend_string *phper_zval_get_string(zval *op);
void phper_zend_string_release(zend_string *s);
zend_long phper_zval_get_long(zval *op);

zend_string *phper_zend_string_init(const char *str, size_t len, int persistent);
zend_string *phper_zend_string_alloc(size_t len, int persistent);
void phper_zend_string_release(zend_string *s);

zval* phper_zend_hash_str_update(HashTable *ht, const char *key, size_t len, zval *pData);
zval* phper_zend_hash_index_update(HashTable *ht, zend_ulong h, zval *pData);
zval* phper_zend_hash_next_index_insert(HashTable *ht, zval *pData);

void phper_array_init(zval *arg);
void *phper_zend_hash_str_find_ptr(const HashTable *ht, const char *str, size_t len);

void phper_zval_obj(zval *z, zend_object *o);
void phper_zval_func(zval *z, zend_function *f);

zend_string *phper_get_function_or_method_name(const zend_function *func);

void phper_zval_ptr_dtor(zval *pDest);

size_t phper_zend_object_properties_size(zend_class_entry *ce);

void *phper_zend_object_alloc(size_t obj_size, zend_class_entry *ce);

zend_object* (**phper_get_create_object(zend_class_entry *ce))(zend_class_entry *class_type);
bool phper_object_init_ex(zval *arg, zend_class_entry *class_type);

bool phper_call_user_function(HashTable *function_table, zval *object, zval *function_name, zval *retval_ptr, uint32_t param_count, zval params[]);

bool phper_zend_hash_str_exists(const HashTable *ht, const char *str, size_t len);
bool phper_zend_hash_index_exists(const HashTable *ht, zend_ulong h);

void phper_zval_ptr_dtor_nogc(zval *zval_ptr);

bool phper_z_refcounted_p(zval *zval_ptr);

zval *phper_execute_data_call_arg(zend_execute_data *execute_data, int index);

#endif //PHPER_PHP_WRAPPER_H
