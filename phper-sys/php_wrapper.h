#ifndef PHPER_PHP_WRAPPER_H
#define PHPER_PHP_WRAPPER_H

#include <php.h>
#include <php_ini.h>
#include <ext/standard/info.h>
#include <zend_exceptions.h>

zend_string *zend_string_init_(const char *str, size_t len, int persistent);
zend_string *zend_new_interned_string_(zend_string *str);
zend_class_entry phper_init_class_entry(const char *class_name, const zend_function_entry *functions);
void phper_zval_string(zval *return_value, const char *s);
zend_uchar phper_zval_get_type(const zval* pz);
void phper_zval_stringl(zval *return_value, const char *s, size_t len);
char *phper_z_strval_p(const zval *v);
zval *phper_get_this(zend_execute_data *execute_data);

#endif //PHPER_PHP_WRAPPER_H
