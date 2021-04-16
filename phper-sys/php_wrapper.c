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