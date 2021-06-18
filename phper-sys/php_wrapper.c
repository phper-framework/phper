#include "php_wrapper.h"

zend_string *phper_zend_new_interned_string(zend_string *str) {
    return zend_new_interned_string(str);
}

zend_class_entry phper_init_class_entry_ex(const char *class_name, size_t class_name_len, const zend_function_entry *functions) {
    zend_class_entry class_container;
    INIT_CLASS_ENTRY_EX(class_container, class_name, class_name_len, functions);
    return class_container;
}

void phper_zval_string(zval *return_value, const char *s) {
    ZVAL_STRING(return_value, s);
}

zend_uchar phper_zval_get_type(const zval* pz) {
    return zval_get_type(pz);
}

void phper_zval_arr(zval *return_value, zend_array *arr) {
    ZVAL_ARR(return_value, arr);
}

void phper_zval_new_arr(zval *return_value) {
    ZVAL_NEW_ARR(return_value);
}

void phper_zval_stringl(zval *return_value, const char *s, size_t len) {
    ZVAL_STRINGL(return_value, s, len);
}

char *phper_z_strval_p(const zval *v) {
    return Z_STRVAL_P(v);
}

zval *phper_get_this(zend_execute_data *execute_data) {
    return getThis();
}

void phper_zval_zval(zval *return_value, zval *zv, int copy, int dtor) {
    ZVAL_ZVAL(return_value, zv, copy, dtor);
}

void phper_zval_dup(zval *return_value, zval *zv) {
    ZVAL_DUP(return_value, zv);
}

void phper_zval_copy(zval *return_value, zval *zv) {
    ZVAL_COPY(return_value, zv);
}

void phper_zval_copy_value(zval *return_value, zval *zv) {
    ZVAL_COPY_VALUE(return_value, zv);
}

zend_string *phper_zval_get_string(zval *op) {
    return zval_get_string(op);
}

zend_long phper_zval_get_long(zval *op) {
    return zval_get_long(op);
}

zend_string *phper_zend_string_init(const char *str, size_t len, int persistent) {
    return zend_string_init(str, len, persistent);
}

zend_string *phper_zend_string_alloc(size_t len, int persistent) {
    return zend_string_alloc(len, persistent);
}

void phper_zend_string_release(zend_string *s) {
    return zend_string_release(s);
}

zval* phper_zend_hash_str_update(HashTable *ht, const char *key, size_t len, zval *pData) {
    return zend_hash_str_update(ht, key, len, pData);
}

zval* phper_zend_hash_index_update(HashTable *ht, zend_ulong h, zval *pData) {
    return zend_hash_index_update(ht, h, pData);
}

zval* phper_zend_hash_next_index_insert(HashTable *ht, zval *pData) {
    return zend_hash_next_index_insert(ht, pData);
}

void phper_array_init(zval *arg) {
    array_init(arg);
}

void *phper_zend_hash_str_find_ptr(const HashTable *ht, const char *str, size_t len) {
    return zend_hash_str_find_ptr(ht, str, len);
}

void phper_zval_obj(zval *z, zend_object *o) {
    ZVAL_OBJ(z, o);
}

void phper_zval_func(zval *z, zend_function *f) {
    ZVAL_FUNC(z, f);
}

#if PHP_VERSION_ID < 80000
static zend_string *phper_zend_string_concat3(
        const char *str1, size_t str1_len,
        const char *str2, size_t str2_len,
        const char *str3, size_t str3_len)
{
    size_t len = str1_len + str2_len + str3_len;
    zend_string *res = zend_string_alloc(len, 0);

    memcpy(ZSTR_VAL(res), str1, str1_len);
    memcpy(ZSTR_VAL(res) + str1_len, str2, str2_len);
    memcpy(ZSTR_VAL(res) + str1_len + str2_len, str3, str3_len);
    ZSTR_VAL(res)[len] = '\0';

    return res;
}
#endif

zend_string *phper_get_function_or_method_name(const zend_function *func) {
    #if PHP_VERSION_ID >= 80000
    return get_function_or_method_name(func);
    #else
    if (func->common.scope) {
        return phper_zend_string_concat3(
                ZSTR_VAL(func->common.scope->name), ZSTR_LEN(func->common.scope->name),
                "::", sizeof("::") - 1,
                ZSTR_VAL(func->common.function_name), ZSTR_LEN(func->common.function_name));
    }
    return func->common.function_name ? zend_string_copy(func->common.function_name) : zend_string_init("main", sizeof("main") - 1, 0);
    #endif
}

void phper_zval_ptr_dtor(zval *pDest) {
    ZVAL_PTR_DTOR(pDest);
}

size_t phper_zend_object_properties_size(zend_class_entry *ce) {
    return zend_object_properties_size(ce);
}

void *phper_zend_object_alloc(size_t obj_size, zend_class_entry *ce) {
    #if PHP_VERSION_ID >= 70300
    return zend_object_alloc(obj_size, ce);
    #else
    void *obj = emalloc(obj_size + zend_object_properties_size(ce));
    memset(obj, 0, obj_size - sizeof(zval));
    return obj;
    #endif
}

zend_object* (**phper_get_create_object(zend_class_entry *ce))(zend_class_entry *class_type) {
    return &ce->create_object;
}

bool phper_object_init_ex(zval *arg, zend_class_entry *class_type) {
    return object_init_ex(arg, class_type) == SUCCESS;
}

bool phper_call_user_function(HashTable *function_table, zval *object, zval *function_name, zval *retval_ptr, uint32_t param_count, zval params[]) {
    function_table = function_table;
    return call_user_function(function_table, object, function_name, retval_ptr, param_count, params) == SUCCESS;
}

bool phper_zend_hash_str_exists(const HashTable *ht, const char *str, size_t len) {
    return zend_hash_str_exists(ht, str, len) != 0;
}

bool phper_zend_hash_index_exists(const HashTable *ht, zend_ulong h) {
    return zend_hash_index_exists(ht, h) != 0;
}

void phper_zval_ptr_dtor_nogc(zval *zval_ptr) {
    zval_ptr_dtor_nogc(zval_ptr);
}

bool phper_z_refcounted_p(zval *zval_ptr) {
    return Z_REFCOUNTED_P(zval_ptr);
}