<?php

ob_start();
phpinfo();
$output = ob_get_contents();
ob_end_clean();
echo substr($output, 0, 100) . '...';
